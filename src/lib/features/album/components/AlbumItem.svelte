<script lang="ts">
	import { type MusicData } from '$lib/features/music/types';
	import { isDesktop, isLinux } from '$lib/platform';
	import { useAlbumItem } from '$lib/features/album/viewmodels/useAlbumItem.svelte';

	interface Props {
		musicList: MusicData[];
		index: number;
	}

	let { musicList, index }: Props = $props();

	const vm = useAlbumItem(musicList, index);
</script>

<div class="col-auto row-[1] h-fit px-3 pb-3">
	<div class="relative w-full">
		{#if vm.isValidFilterAlbum}
			<div
				class="absolute left-0 top-0 z-10 h-full w-full
            rounded-lg shadow-[inset_0_0_0_2px_white]"
			></div>
		{:else}
			<div
				class="album-item-actions absolute z-20 h-full w-full cursor-pointer
                rounded-lg bg-white/20 shadow-[inset_0_0_0_2px_white]"
				onclick={vm.setFilterAlbum}
			></div>
		{/if}
		{#await vm.albumImage}
			<div class="aspect-square w-full"></div>
		{:then image}
			{#if image}
				<img
					class="aspect-square w-full rounded-lg object-cover {isDesktop() &&
						!isLinux() &&
						'animate__animated animate__fadeIn'}"
					src={image}
					alt="Album"
				/>
			{:else}
				<div class="aspect-square w-full rounded-lg"></div>
			{/if}
		{/await}
	</div>
	<p
		class="animate-scroll-overflow-text mt-2 overflow-hidden whitespace-nowrap font-medium md:text-lg"
	>
		{vm.music.album}
	</p>
	<p
		class="text-opacity-background-80 animate-scroll-overflow-text overflow-hidden whitespace-nowrap text-[15px] md:text-base"
	>
		{vm.music.albumArtist ?? vm.music.artist}
	</p>
</div>

<style lang="scss">
	.album-item-actions {
		opacity: 0;

		&:hover {
			animation-name: fadeIn;
			animation-duration: 0.5s;
			animation-fill-mode: forwards;
		}
	}
</style>
