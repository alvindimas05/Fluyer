<script lang="ts">
	import type { MusicData } from '$lib/features/music/types';

	interface Props {
		music: MusicData;
		uuid: string;
		visible?: boolean;
	}

	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import musicStore from '$lib/stores/music.svelte';
	import MetadataService from '$lib/services/MetadataService.svelte';
	import QueueService from '$lib/services/QueueService.svelte';

	let { music, uuid, visible = true }: Props = $props();

	let index = $derived(musicStore.queueIds.indexOf(uuid));
	let isPlaying = $derived(musicStore.currentIndex === index);
	let isPrevious = $derived(index < musicStore.currentIndex);
	let albumImage = $state<Promise<string | null> | null>(null);
	let currentBlobUrl: string | null = null;

	// Fetch album image only when visible, with blob URL cleanup
	$effect(() => {
		// Track dependencies synchronously
		const currentMusic = music;
		const isVisible = visible;

		if (!isVisible) return;

		let cancelled = false;

		(async () => {
			const imagePromise = MetadataService.getMusicCoverArt(currentMusic);
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
			if (currentBlobUrl) {
				URL.revokeObjectURL(currentBlobUrl);
				currentBlobUrl = null;
			}
		};
	});

	function removePlaylist() {
		QueueService.remove(index);
	}

	function goToPlaylist() {
		QueueService.goTo(index);
	}
</script>

<div class="relative">
	<div class="relative grid grid-cols-[max-content_auto_max-content] px-3 py-2">
		<div
			class="h-11 w-11 md:h-12 md:w-12 lg:h-14 lg:w-14 md-hdpi:h-11 md-hdpi:w-11 lg-hdpi:h-12 lg-hdpi:w-12"
		>
			{#await albumImage}
				<div class="aspect-square w-full"></div>
			{:then image}
				<img class="aspect-square w-full rounded" src={image} alt="Album" />
			{/await}
		</div>
		<div class="ms-3 text-sm md:text-base">
			<p class="font-medium">{music.title}</p>
			<p class="text-opacity-background-80">{music.artist}</p>
		</div>
		<div
			class="h-11 w-11 md:h-12 md:w-12 lg:h-14 lg:w-14 md-hdpi:h-11 md-hdpi:w-11 lg-hdpi:h-12 lg-hdpi:w-12"
		></div>
	</div>
	{#if isPlaying}
		<div
			class="absolute left-0 top-0 z-10 grid w-full grid-cols-[max-content_auto_max-content] px-3 py-2"
		>
			<div
				class="aspect-square h-11 w-11 md:h-12 md:w-12 lg:h-14 lg:w-14 md-hdpi:h-11 md-hdpi:w-11 lg-hdpi:h-12 lg-hdpi:w-12"
			></div>
			<div></div>
			<div
				class="aspect-square h-11 w-11 p-1 md:h-12 md:w-12 lg:h-14 lg:w-14 lg:p-3 md-hdpi:h-11 md-hdpi:w-11 lg-hdpi:h-12 lg-hdpi:w-12"
			>
				<div class="animate__animated animate__infinite animate__pulse w-3/4 md:w-full">
					<Icon type={IconType.Playing} />
				</div>
			</div>
		</div>
	{:else}
		<div
			class="playlist-item-controls absolute left-0 top-0 z-10 grid w-full grid-cols-[max-content_auto_max-content] px-3 py-2"
		>
			<button
				class="aspect-square h-11 w-11 md:h-12 md:w-12 lg:h-14 lg:w-14 md-hdpi:h-11 md-hdpi:w-11 lg-hdpi:h-12 lg-hdpi:w-12"
				onclick={goToPlaylist}
				onpointerdown={(e) => e.stopPropagation()}
			>
				{#if !isPlaying}
					<div class="h-full w-full rounded bg-black bg-opacity-40 lg:p-1">
						{#if isPrevious}
							<Icon type={IconType.Previous} />
						{:else}
							<Icon type={IconType.Next} />
						{/if}
					</div>
				{/if}
			</button>
			<div class="cursor-grab"></div>
			<button
				class="aspect-square h-11 w-11 md:h-12 md:w-12 lg:h-14 lg:w-14 lg:p-1 md-hdpi:h-11 md-hdpi:w-11 lg-hdpi:h-12 lg-hdpi:w-12"
				onclick={removePlaylist}
				onpointerdown={(e) => e.stopPropagation()}
			>
				<Icon type={IconType.Remove} />
			</button>
		</div>
	{/if}
</div>

<style lang="scss">
	.playlist-item-controls {
		opacity: 0;

		&:hover {
			animation-name: fadeIn;
			animation-duration: 0.5s;
			animation-fill-mode: forwards;
		}
	}
</style>
