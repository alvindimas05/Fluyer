<script lang="ts">
import type { MusicData } from "../music/types";
import { musicAlbumList, musicList } from "$lib/stores/music";
import AlbumItem from "./AlbumItem.svelte";
import MusicController from "$lib/controllers/MusicController";
import { swipeMinimumTop } from "$lib/stores";
import { onMount } from "svelte";
import type { Unsubscriber } from "svelte/store";
import { afterNavigate } from "$app/navigation";
import { isMobile } from "$lib/platform";
import { mobileStatusBarHeight } from "$lib/stores/mobile";

let element: HTMLDivElement;
let unsubscribeMusicList: Unsubscriber;

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
	MusicController.setMusicAlbumList(groupByAlbum());
	unsubscribeMusicList = musicList.subscribe(() => {
		MusicController.setMusicAlbumList(groupByAlbum());
		setTimeout(() => ($swipeMinimumTop = element.clientHeight), 0);
	});
});

afterNavigate(() => {
	unsubscribeMusicList();
});
</script>

<div
    class="grid auto-cols-[50%] sm:auto-cols-[33.3334%]
        md-mdpi:auto-cols-[25%] lg-mdpi:auto-cols-[20%] xl-mdpi:auto-cols-[16.6667%]
        md-hdpi:auto-cols-[25%] lg-hdpi:auto-cols-[20%]
        grid-rows-[1fr] w-full overflow-x-auto scrollbar-hidden"
    style={`padding-top: ${isMobile() ? $mobileStatusBarHeight : 44}px`}
    bind:this={element}
    onwheel={onMouseWheel}>
    {#each Object.entries($musicAlbumList) as [_, musicList], index}
        <AlbumItem {musicList} {index} />
    {/each}
</div>
