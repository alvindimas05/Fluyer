<script lang="ts">
	import { isAndroid, isDesktop, isMacos, isMobile } from '$lib/platform';
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import View from '$lib/ui/components/View.svelte';
	import ProgressBar from '$lib/ui/components/ProgressBar.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import ProgressService from '$lib/services/ProgressService.svelte';
	import { MusicConfig } from '$lib/constants/MusicConfig';
	import MetadataService from '$lib/services/MetadataService.svelte';
	import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
	import PageService from '$lib/services/PageService.svelte';
	import settingStore from '$lib/stores/setting.svelte';
	import showThenFade from '$lib/actions/showThenFade';
	import { RepeatMode } from '$lib/features/music/types';
	import QueueService from '$lib/services/QueueService.svelte';
	import LibraryService from '$lib/services/LibraryService.svelte';
	import LyricService, { type MusicLyric } from '$lib/services/LyricService.svelte';

	let music = $derived(musicStore.currentMusic);
	let progressPercentage = $derived(0);
	let progressDurationText = $derived('');
	let progressDurationNegativeText = $derived('');
	let albumImage = $derived(MetadataService.getMusicCoverArt(musicStore.currentMusic));

	let lyrics: MusicLyric[] = $state([]);
	let selectedLyricIndex = $state(0);
	let volumePercentage = $state(musicStore.volume);

	let updateProgressText = $state(true);
	let shouldAnimate = isDesktop() && !isMobile();

	function handleButtonPlayPause() {
		if (musicStore.isPlaying) {
			MusicPlayerService.pause();
		} else {
			MusicPlayerService.play();
		}
	}

	function handleButtonPrevious() {
		MusicPlayerService.previous();
	}

	function handleButtonNext() {
		MusicPlayerService.next();
	}

	function handleButtonBack() {
		PageService.back();
	}

	async function handleButtonShuffle() {
		await MusicPlayerService.pause();

		await QueueService.resetAndAddList(await LibraryService.shuffleMusicList(musicStore.queue));

		await MusicPlayerService.play();
	}
	async function onKeyDown(
		e: KeyboardEvent & {
			currentTarget: EventTarget & Document;
		}
	) {
		if (e.key === 'Escape') handleButtonBack();
	}

	async function resetLyrics() {
		selectedLyricIndex = 0;
		lyrics = [];

		if (!musicStore.currentMusic) return;
		const resLyrics = await LyricService.get(music);
		if (resLyrics == null) {
			lyrics = [];
			return;
		}
		lyrics = resLyrics;
	}

	function resetSelectedLyricIndex() {
		if (lyrics.length < 1) return;

		const duration = musicStore.progressDuration / 1000;
		if (duration < lyrics[0].duration) {
			scrollToSelectedLyric();
			return;
		}
		for (let i = 0; i < lyrics.length; i++) {
			if (duration < lyrics[i].duration) {
				selectedLyricIndex = i - 1;
				scrollToSelectedLyric();
				return;
			}
		}
		selectedLyricIndex = lyrics.length - 1;
		scrollToSelectedLyric();
	}

	function refreshProgressText() {
		if (!updateProgressText) return;
		progressDurationText = ProgressService.formatDuration(musicStore.progressDuration);
		progressDurationNegativeText =
			'-' +
			ProgressService.formatDuration(
				(musicStore.currentMusic?.duration ?? 0) - musicStore.progressDuration
			);
	}

	function handleProgressClick(percentage: number) {
		MusicPlayerService.seekByPercentage(percentage);
	}

	function handleProgressEnter() {
		updateProgressText = false;
	}

	function handleProgressMove(percentage: number) {
		updateProgressText = false;
		progressDurationText = ProgressService.formatDuration(
			(musicStore.currentMusic?.duration ?? 0) * (percentage / 100)
		);
		progressDurationNegativeText =
			'-' +
			ProgressService.formatDuration(
				(musicStore.currentMusic?.duration ?? 0) * ((100 - percentage) / 100)
			);
	}

	function handleProgressLeave() {
		updateProgressText = true;
		refreshProgressText();
	}

	function handleVolumeProgressClick(percentage: number) {
		musicStore.volume = percentage / 100;
	}

	function scrollToSelectedLyric() {
		document.getElementById('selected-lyric')?.scrollIntoView({
			block: window.innerWidth > 768 ? 'center' : 'start',
			behavior: 'smooth'
		});
	}

	$effect(() => {
		progressPercentage = musicStore.progressPercentage;
		refreshProgressText();
		resetSelectedLyricIndex();
	});

	$effect(() => {
		musicStore.currentIndex;
		albumImage = MetadataService.getMusicCoverArt(musicStore.currentMusic);
		resetLyrics();
	});

	$effect(() => {
		volumePercentage = musicStore.volumePercentage;
	});
</script>

<svelte:document onkeydown={onKeyDown} />

<div
	class="mx-auto grid h-full w-full max-w-[35rem] md:max-w-none md:gap-y-0 md:pt-0
    {lyrics.length > 1
		? 'md:grid-cols-[40%_55%]'
		: 'root-nolyrics justify-center md:grid-cols-[50%]'}
    {isMacos() && 'pt-6'}"
>
	<div
		class="md:col-[1] md:row-[1] {isMobile() ? 'p-5' : 'p-4'} flex items-end md:p-0
        {lyrics.length > 1 ? 'justify-end' : 'justify-center'}"
	>
		<div
			class="w-full {lyrics.length > 0 && 'ms-auto'}
            md-mdpi:w-[80%] lg-mdpi:w-[75%] xl-mdpi:w-[65%]
            md-hdpi:w-[90%] lg-hdpi:w-[80%] xl-hdpi:w-[70%]
            md-xhdpi:w-[80%] lg-xhdpi:w-[70%]"
		>
			{#await albumImage}
				<div class="aspect-square w-full"></div>
			{:then image}
				<img
					class="aspect-square w-full rounded-lg object-cover shadow-lg {shouldAnimate &&
						'animate__animated animate__fadeIn'}"
					src={image}
					alt="Music Album"
				/>
			{/await}
		</div>
	</div>
	<div
		class="order-last md:order-2 md:col-[1] md:row-[2] {isMobile()
			? 'px-5'
			: 'px-4'} pb-5 pt-2 {isMobile() && 'mb-5'}
        flex md:p-0 md:pb-0 {lyrics.length > 0 ? 'justify-end' : 'justify-center'}"
	>
		<View
			class="h-fit w-full rounded-xl px-4
            py-5 hover:px-5 hover:py-7 md:mt-4
            md-mdpi:w-[80%] lg-mdpi:w-[75%] xl-mdpi:w-[65%]
            md-hdpi:w-[90%] lg-hdpi:w-[80%] xl-hdpi:w-[70%]
            md-xhdpi:w-[80%] lg-xhdpi:w-[70%]"
		>
			<div class="grid w-full grid-cols-[auto,1fr,auto]">
				<div class="flex w-12 text-xs lg-mdpi:text-sm">
					<span class="self-end opacity-75">{progressDurationText}</span>
				</div>
				<div
					class="mt-2 overflow-hidden text-center text-sm font-medium
                    opacity-90 md-mdpi:text-base md-hdpi:text-base md-xhdpi:text-sm lg-xhdpi:text-base"
				>
					<!-- Note: Idk why the title scroll doesn't work without sacrificing first element -->
					<p class="animate-scroll-overflow-text"></p>
					<p class="animate-scroll-overflow-text overflow-x-hidden whitespace-nowrap">
						{music?.albumArtist ?? music?.artist ?? MusicConfig.defaultArtist}
						{MusicConfig.separator}
						{music?.title ?? MusicConfig.defaultTitle}
					</p>
				</div>
				<div class="flex w-12 justify-end text-xs lg-mdpi:text-sm">
					<span class="self-end opacity-75">{progressDurationNegativeText}</span>
				</div>
			</div>
			<div class="w-full pb-2 pt-4">
				<ProgressBar
					bind:value={musicStore.progressValue}
					min={MusicConfig.min}
					max={MusicConfig.max}
					step={MusicConfig.step}
					{progressPercentage}
					onProgressClick={handleProgressClick}
					onProgressEnter={handleProgressEnter}
					onProgressMove={handleProgressMove}
					onProgressLeave={handleProgressLeave}
					size="md"
				/>
			</div>
			<div
				class="mt-4 grid w-full items-center gap-2
                {isAndroid() || !settingStore.ui.play.showBackButton
					? 'grid-cols-[1fr_auto_auto_auto_1fr]'
					: 'grid-cols-7'}"
			>
				{#if !isAndroid() && settingStore.ui.play.showBackButton}
					<div class="flex items-center">
						<button
							id="btn-back"
							class="animate__animated show-then-fade mx-2 w-7 md-mdpi:w-8 md-hdpi:w-8"
							use:showThenFade
							onclick={handleButtonBack}><Icon type={IconType.PlayBack} /></button
						>
					</div>
				{/if}
				<div class="flex justify-end">
					{#if settingStore.ui.showRepeatButton}
						<button
							class="mx-2 w-7 md-mdpi:w-8 md-hdpi:w-8 {musicStore.repeatMode === RepeatMode.None
								? 'opacity-80'
								: ''}"
							onclick={MusicPlayerService.toggleRepeatMode}
						>
							{#if musicStore.repeatMode === RepeatMode.All}
								<Icon type={IconType.Repeat} />
							{:else if musicStore.repeatMode === RepeatMode.None}
								<Icon type={IconType.RepeatPlayNone} />
							{:else if musicStore.repeatMode === RepeatMode.One}
								<Icon type={IconType.RepeatOne} />
							{/if}
						</button>
					{/if}
				</div>
				<div class="flex justify-end">
					<button
						class="w-12 sm:w-14 md-mdpi:w-12 lg-mdpi:w-14 md-hdpi:w-12"
						onclick={handleButtonPrevious}><Icon type={IconType.Previous} /></button
					>
				</div>
				<div class="flex justify-center">
					<button
						class="w-12 sm:w-14 md-mdpi:w-12 lg-mdpi:w-14 md-hdpi:w-12"
						onclick={handleButtonPlayPause}
					>
						{#if musicStore.isPlaying}
							<Icon type={IconType.Pause} />
						{:else}
							<Icon type={IconType.Play} />
						{/if}
					</button>
				</div>
				<div class="flex justify-start">
					<button
						class="w-12 sm:w-14 md-mdpi:w-12 lg-mdpi:w-14 md-hdpi:w-12"
						onclick={handleButtonNext}><Icon type={IconType.Next} /></button
					>
				</div>
				<div class="flex justify-start">
					{#if settingStore.ui.showShuffleButton}
						<button class="mx-2 w-7 md-mdpi:w-8 md-hdpi:w-8" onclick={handleButtonShuffle}>
							<Icon type={IconType.Shuffle} />
						</button>
					{/if}
				</div>
			</div>
			{#if settingStore.ui.play.showVolume && !settingStore.bitPerfectMode}
				<div id="volume-bar" class="mt-5">
					<div class="grid grid-cols-[auto_1fr_auto] items-center gap-3">
						<button class="w-5" onclick={() => (musicStore.volume = 0)}>
							<Icon type={IconType.Mute} />
						</button>
						<div class="relative">
							<ProgressBar
								bind:value={musicStore.volume}
								progressPercentage={volumePercentage}
								onProgressClick={handleVolumeProgressClick}
								min={MusicConfig.vmin}
								max={MusicConfig.vmax}
								step={MusicConfig.vstep}
								showTooltip={false}
								size="sm"
							/>
						</div>
						<button class="w-5" onclick={() => (musicStore.volume = 1)}>
							<Icon type={IconType.Speaker} />
						</button>
					</div>
				</div>
			{/if}
		</View>
	</div>
	{#if lyrics.length > 0}
		<div
			class="scrollbar-hidden animate__animated animate__faster animate__fadeInUp w-full overflow-y-auto overflow-x-hidden
            [mask-image:linear-gradient(to_bottom,rgba(0,0,0,1)_60%,rgba(0,0,0,0))] md:col-[2]
            md:row-[1/span_2]
            md:h-screen md:px-20 md:[mask-image:linear-gradient(to_bottom,rgba(0,0,0,0),rgba(0,0,0,1),rgba(0,0,0,0))] {isMobile()
				? 'px-5'
				: 'px-4'}"
		>
			<div class="flex">
				<div
					id="lyrics"
					class="sm-mdpi:text-[1.25rem] h-full w-full text-[1.15rem] font-bold md:my-[40vh]
                    md:w-[55vw] md-mdpi:text-[1.4rem] lg-mdpi:text-[1.7rem]
                    md-hdpi:text-[1.3rem] lg-hdpi:text-[1.45rem]
                    md-xhdpi:text-[1.2rem] lg-xhdpi:text-[1.4rem]"
				>
					{#each lyrics as lyric, i}
						<div
							id={selectedLyricIndex === i ? 'selected-lyric' : ''}
							class={selectedLyricIndex === i
								? `sm-mdpi:text-[1.35rem]
                                    py-5 text-[1.25rem] md:py-7
                                    lg:py-10 md-mdpi:text-[1.55rem]
                                    lg-mdpi:text-[1.85rem] md-hdpi:text-[1.4rem]
                                    lg-hdpi:text-[1.6rem] md-xhdpi:text-[1.3rem] lg-xhdpi:text-[1.55rem]`
								: 'py-5 opacity-50 md:py-7 lg:py-10'}
						>
							{#if lyric.value.length > 0}
								{lyric.value}
							{:else}
								<div
									class={selectedLyricIndex === i
										? 'w-[1.4rem] md:w-[1.9rem] lg:w-[2.25rem]'
										: 'w-[1.25rem] md:w-[1.75rem] lg:w-[2.15rem]'}
								>
									<Icon type={IconType.Note} />
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>
