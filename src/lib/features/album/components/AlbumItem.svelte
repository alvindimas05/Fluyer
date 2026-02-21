<script lang="ts">
	import { type MusicData } from '$lib/features/music/types';
	import { useAlbumItem } from '$lib/features/album/viewmodels/useAlbumItem.svelte';

	interface Props {
		musicList: MusicData[];
		index: number;
		visible?: boolean;
	}

	let { musicList, index, visible = true }: Props = $props();

	const vm = useAlbumItem(
		() => musicList,
		() => index,
		() => visible
	);
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
					class="animate__animated animate__fadeIn aspect-square w-full rounded-lg object-cover"
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
		transition: opacity 0.75s;

		&:hover {
			opacity: 1;
		}
	}
</style>
