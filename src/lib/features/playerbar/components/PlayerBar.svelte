<script lang="ts">
	import { PageRoutes } from '$lib/constants/PageRoutes';
	import { IconType } from '$lib/ui/icon/types';
	import { MusicConfig } from '$lib/constants/MusicConfig';
	import { type MusicData, RepeatMode } from '$lib/features/music/types';
	import MetadataService from '$lib/services/MetadataService.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import settingStore from '$lib/stores/setting.svelte';
	import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
	import ProgressService from '$lib/services/ProgressService.svelte';
	import PageService from '$lib/services/PageService.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import ProgressBar from '$lib/ui/components/ProgressBar.svelte';
	import View from '$lib/ui/components/View.svelte';
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { onDestroy, onMount } from 'svelte';
	import QueueService from '$lib/services/QueueService.svelte';
	import LibraryService from '$lib/services/LibraryService.svelte';

	let element: HTMLDivElement;
	let oldMusic: MusicData | undefined = $state(undefined);
	let title = $state(MusicConfig.defaultTitle);
	let artist = $state(MusicConfig.defaultArtist);
	let albumImage = $state<Promise<string | null> | null>(null);
	let currentBlobUrl: string | null = null;

	// Fetch album image with blob URL cleanup
	$effect(() => {
		musicStore.currentMusic;
		let cancelled = false;

		(async () => {
			const imagePromise = MetadataService.getMusicCoverArt(musicStore.currentMusic);
			albumImage = imagePromise;

			const url = await imagePromise;
			if (!cancelled && url && url.startsWith('blob:')) {
				if (currentBlobUrl) {
					URL.revokeObjectURL(currentBlobUrl);
				}
				currentBlobUrl = url;
			}
		})();

		return () => {
			cancelled = true;
		};
	});

	onDestroy(() => {
		if (currentBlobUrl) {
			URL.revokeObjectURL(currentBlobUrl);
			currentBlobUrl = null;
		}
	});

	let isPlaying = $derived(musicStore.isPlaying);
	let progressPercentage = $state(0);
	let volumePercentage = $state(0);

	const gridRight = (() => {
		if (settingStore.ui.showRepeatButton && settingStore.ui.showShuffleButton)
			return 'grid-cols-[repeat(5,auto)]';
		if (settingStore.ui.showRepeatButton && settingStore.ui.showShuffleButton)
			return 'grid-cols-[repeat(4,auto)]';
		return 'grid-cols-[repeat(3,auto)]';
	})();

	function handleButtonPlayPause() {
		if (musicStore.isPlaying) {
			musicStore.isPlaying = false;
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

	async function handleButtonShuffle() {
		await MusicPlayerService.pause();

		await QueueService.resetAndAddList(await LibraryService.shuffleMusicList(musicStore.queue));

		await MusicPlayerService.play();
		ProgressService.start();
	}

	function redirectToPlay() {
		PageService.goTo(PageRoutes.PLAY);
	}

	function handleVolumeButton() {
		musicStore.volume = musicStore.volume > 0 ? 0 : 1;
	}

	function refresh() {
		let music = musicStore.currentMusic;

		if (!music) {
			title = MusicConfig.defaultTitle;
			artist = MusicConfig.defaultArtist;
			return;
		}

		if (oldMusic && oldMusic.path === music.path) return;

		oldMusic = music;
		title = music.title!;
		artist = music.artist;
	}

	function handleProgressClick(percentage: number) {
		MusicPlayerService.seekByPercentage(percentage);
	}

	function handleVolumeProgressClick(percentage: number) {
		musicStore.volume = percentage / 100;
	}

	function updatePlayerBarHeight() {
		playerBarStore.height = element.offsetHeight;
	}

	$effect(() => {
		progressPercentage = musicStore.progressPercentage;
	});

	$effect(() => {
		volumePercentage = musicStore.volumePercentage;
	});

	$effect(() => {
		musicStore.currentIndex;
		musicStore.list;
		refresh();
	});

	onMount(() => {
		updatePlayerBarHeight();
	});
</script>

<svelte:window onresize={updatePlayerBarHeight} />

<div
	class="animate__animated animate__slideInUp w-full px-3 pt-3"
	style="padding-bottom: {mobileStore.navigationBarHeight > 12
		? mobileStore.navigationBarHeight
		: 12}px;"
	bind:this={element}
>
	<ProgressBar
		bind:value={musicStore.progressValue}
		{progressPercentage}
		onProgressClick={handleProgressClick}
		min={MusicConfig.min}
		max={MusicConfig.max}
		step={MusicConfig.step}
		showTooltip={true}
		tooltipFormatter={(percentage) =>
			ProgressService.formatDuration((musicStore.currentMusic?.duration ?? 0) * (percentage / 100))}
		class="mb-3"
		size="lg"
	/>

	<View class="rounded-full px-3 py-2 hover:px-4 hover:py-3">
		<div class="grid w-full grid-cols-[auto_min-content] py-1 md:grid-cols-3">
			<div class="flex items-center ps-1 sm:gap-x-1">
				<button class="hidden w-10 sm:block md:w-12 lg:w-12" onclick={handleButtonPrevious}
					><Icon type={IconType.Previous} /></button
				>
				<button class="w-10 md:w-12 lg:w-12" onclick={handleButtonPlayPause}>
					{#if isPlaying}
						<Icon type={IconType.Pause} />
					{:else}
						<Icon type={IconType.Play} />
					{/if}
				</button>
				<button class="hidden w-10 sm:block md:w-12 lg:w-12" onclick={handleButtonNext}
					><Icon type={IconType.Next} /></button
				>
			</div>
			<div
				class="order-first ms-2 items-center justify-center text-sm
                md:order-none md:ms-0
                md:flex md:text-[15px]"
			>
				<div class="grid grid-cols-[2.5rem_auto] md:grid-cols-[3rem_auto]">
					<button onclick={redirectToPlay}>
						{#await albumImage}
							<div class="aspect-square w-full"></div>
						{:then image}
							<img
								class="animate__animated animate__fadeIn aspect-square w-full rounded object-cover"
								src={image}
								alt="Album"
							/>
						{/await}
					</button>
					<div class="ms-3 grid grid-rows-[auto_1fr_1fr] overflow-hidden">
						<!-- Note: Idk why the title scroll doesn't work without sacrificing first element -->
						<p class="animate-scroll-overflow-text"></p>
						<p class="animate-scroll-overflow-text overflow-x-hidden whitespace-nowrap font-medium">
							{title}
						</p>
						<p
							class="animate-scroll-overflow-text overflow-x-hidden whitespace-nowrap text-opacity-80"
						>
							{artist}
						</p>
					</div>
				</div>
			</div>
			<div class="hidden justify-end md:grid">
				<div class="grid items-center gap-3 {gridRight}">
					{#if settingStore.ui.showRepeatButton}
						<button
							class="w-6 {musicStore.repeatMode === RepeatMode.None ? 'opacity-60' : ''}"
							onclick={MusicPlayerService.toggleRepeatMode}
						>
							{#if musicStore.repeatMode === RepeatMode.All}
								<Icon type={IconType.Repeat} />
							{:else if musicStore.repeatMode === RepeatMode.None}
								<Icon type={IconType.RepeatNone} />
							{:else if musicStore.repeatMode === RepeatMode.One}
								<Icon type={IconType.RepeatOne} />
							{/if}
						</button>
					{/if}
					{#if settingStore.ui.showShuffleButton}
						<button class="w-6" onclick={handleButtonShuffle}>
							<Icon type={IconType.Shuffle} />
						</button>
					{/if}
					<button
						class="w-6 {settingStore.bitPerfectMode ? 'pointer-events-none' : ''}"
						onclick={handleVolumeButton}
					>
						{#if volumePercentage > 0}
							<Icon type={IconType.Speaker} />
						{:else}
							<Icon type={IconType.Mute} />
						{/if}
					</button>
					<div class="relative w-24 {settingStore.bitPerfectMode ? 'pointer-events-none' : ''}">
						<ProgressBar
							bind:value={musicStore.volume}
							progressPercentage={volumePercentage}
							onProgressClick={handleVolumeProgressClick}
							min={MusicConfig.vmin}
							max={MusicConfig.vmax}
							step={MusicConfig.vstep}
							showTooltip={false}
							class="w-24"
							size="sm"
						/>
					</div>
				</div>
			</div>
		</div>
	</View>
</div>
