<script lang="ts">
import type { MusicData } from "../music/types";
import { musicList } from "$lib/stores/music";
import AlbumItem from "./AlbumItem.svelte";
import MusicController from "$lib/controllers/MusicController";
import { swipeMinimumTop } from "$lib/stores";
import { onMount } from "svelte";
    import { platform } from "@tauri-apps/plugin-os";
    import { isDesktop, isMobile } from "$lib/platform";

let element: HTMLDivElement;
let grouppedAlbums = $state(groupByAlbum());

function groupByAlbum(): MusicData[][] {
	const albumsMap = MusicController.musicList()!.reduce(
		(acc, item) => {
			if (item.album === null || item.album.trim() === "") {
				return acc;
			}

			if (!acc[item.album]) {
				acc[item.album] = [];
			}

			acc[item.album].push(item);

			return acc;
		},
		{} as Record<string, MusicData[]>,
	);

	return Object.values(albumsMap);
}

function onMouseWheel(
	e: WheelEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	if (e.deltaX == -0) {
		e.preventDefault();
		element.scrollLeft += e.deltaY;
	}
}

onMount(() => {
	musicList.subscribe(() => {
		grouppedAlbums = groupByAlbum();
		setTimeout(() => {
			$swipeMinimumTop = element.clientHeight;
		}, 1);
	});
});
</script>

<div
    class={`grid auto-cols-[50%] sm:auto-cols-[33.3334%] md:auto-cols-[25%] tb:auto-cols-[20%] lg:auto-cols-[20%]
    xl:auto-cols-[16.6667%] grid-rows-[1fr] w-full overflow-x-auto scrollbar-hidden
    ${isDesktop() && "pt-8"}`}
    bind:this={element}
    onwheel={onMouseWheel}
>
    {#each Object.entries(grouppedAlbums) as [album, musicList], index}
        <AlbumItem {musicList} {index} />
    {/each}
</div>
