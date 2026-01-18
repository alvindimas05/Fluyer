<script lang="ts">
	import { type MusicData, type FolderData } from '../types';
	import { useMusicItem } from '../viewmodels/useMusicItem.svelte';
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import { isDesktop, isLinux } from '$lib/platform';
	import { MusicConfig } from '$lib/constants/MusicConfig';

	interface Props {
		music: MusicData;
		folder?: FolderData;
	}

	let { music, folder }: Props = $props();

	const vm = useMusicItem(music, folder);
	const shouldAnimate = isDesktop() && !isLinux();
</script>

<div class="relative text-sm md:text-base">
	<div class="grid grid-cols-[max-content_auto_max-content] py-2">
		{#await vm.albumImage}
			<div class="relative aspect-square h-12 w-12 md:h-14 md:w-14"></div>
		{:then image}
			{#if image && !folder}
				<img
					class="relative h-12 w-12 rounded object-cover md:h-14 md:w-14 {shouldAnimate &&
						'animate__animated animate__fadeIn'}"
					src={image}
					alt="Album"
				/>
			{:else if image && folder}
				<!-- Folder with album art: folder icon behind, album image centered on top -->
				<div class="relative aspect-square h-12 w-12 md:h-14 md:w-14">
					<div class="absolute inset-0 opacity-75">
						<Icon type={IconType.Folder} />
					</div>
					<div class="absolute inset-0 flex items-center justify-center">
						<img
							class="h-4 w-4 rounded-sm object-cover shadow-md md:h-5 md:w-5 mt-2 {shouldAnimate &&
								'animate__animated animate__fadeIn'}"
							src={image}
							alt="Album"
						/>
					</div>
				</div>
			{:else}
				<div class="relative aspect-square h-12 w-12 md:h-14 md:w-14"></div>
			{/if}
		{/await}

		<div class="ms-3 overflow-hidden">
			<p
				class="animate-scroll-overflow-text overflow-hidden whitespace-nowrap text-sm/[14px] font-medium md:text-sm"
			>
				{vm.titleLabel}
			</p>
			<p
				class="text-opacity-background-90 animate-scroll-overflow-text overflow-hidden whitespace-nowrap pt-[4px] text-xs/[14px] md:pt-0 md:text-xs"
			>
				{vm.mediumLabel}
			</p>
			<p class="text-opacity-background-90 mt-[2px] text-xs/[14px] md:text-xs">
				{vm.smallLabel}
			</p>
		</div>

		<div class="h-12 w-12 ps-2 md:h-14 md:w-14"></div>
	</div>

	<div class="absolute left-0 top-0 w-full py-2">
		<div class="music-item-play grid w-full grid-cols-[max-content_auto_max-content]">
			<button class="h-12 w-12 md:h-14 md:w-14" onclick={vm.addMusicAndPlay}>
				<div
					class="box-border grid items-center justify-items-center rounded bg-black bg-opacity-40 md:p-1"
				>
					<Icon type={IconType.Play} />
				</div>
			</button>

			<div class={folder ? 'cursor-pointer' : 'cursor-default'} onclick={vm.selectFolder}></div>

			<div class="h-12 w-12 ps-4 md:h-14 md:w-14">
				<button class="aspect-square h-full w-full" onclick={vm.addMusic}>
					<Icon type={IconType.QueueMusic} />
				</button>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	.music-item-play {
		opacity: 0;

		&:hover {
			animation-name: fadeIn;
			animation-duration: 0.5s;
			animation-fill-mode: forwards;
		}
	}
</style>
