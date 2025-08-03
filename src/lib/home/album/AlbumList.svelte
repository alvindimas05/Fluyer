<script lang="ts">
import type { MusicData } from "$lib/home/music/types";
import { musicAlbumList, musicList } from "$lib/stores/music";
import AlbumItem from "./AlbumItem.svelte";
import MusicController from "$lib/controllers/MusicController";
import {onDestroy, onMount} from "svelte";
import { isMobile } from "$lib/platform";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import { swipeMinimumTop } from "$lib/stores";
import type {Unsubscriber} from "svelte/store";
import { VList } from "virtua/svelte";
import {filterAlbum, filterSearch} from "$lib/stores/filter";

// Responsive rules: [minWidth, minDppx, widthPercent]]
// md-hdpi:auto-cols-[20%] lg-hdpi:auto-cols-[16.6667%]
// md-mdpi:auto-cols-[20%] lg-mdpi:auto-cols-[16.6667%] xl-mdpi:auto-cols-[12.5%]
// auto-cols-[50%] sm:auto-cols-[33.3334%]
const rules = [
    // xhdpi (DPR > 2.0)
    [1024, 2.01, 0.166667], // lg-xhdpi → 16.6667%
    [768, 2.01, 0.25], // md-xhdpi → 25%

    // hdpi (1.01 ≤ DPR ≤ 2.0)
    [1280, 1.01, 0.166667], // xl-hdpi → 12.5%
    [1024, 1.01, 0.2], // lg-hdpi → 16.6667%
    [768, 1.01, 0.25], // md-hdpi → 20%

    // default (DPR <= 1.0)
    [1536, 0, 0.125], // 2xl → 12.5%
    [1280, 0, 0.166667], // xl → 16.6667%
    [1024, 0, 0.2], // lg → 20%
    [768, 0, 0.333334], // md → 33.3334%
];
let itemElementHeight = $state(0);
let itemWidth = $state(0.5 * window.innerWidth);
let itemHeight = $state(0);
let paddingTop = $derived((isMobile() ? $mobileStatusBarHeight : 0) + 44);

function updateSize(){
    updateItemWidth();
    itemHeight = itemElementHeight;
}

function updateItemWidth() {
    const w = window.innerWidth;
    const dpi = window.devicePixelRatio;

    for (const [minW, minDppx, width] of rules) {
        if (w >= minW && dpi >= minDppx){
            itemWidth = width * window.innerWidth;
            return;
        }
    }
    itemWidth = 0.5 * window.innerWidth;
}

let data = $derived.by(() => {
    if (!Array.isArray($musicAlbumList)) return [];

    const search = $filterSearch.toLowerCase();
    if (!search) return $musicAlbumList;

    return $musicAlbumList.filter((musicList) => {
        return (
            ($filterAlbum && musicList[0].album === $filterAlbum.name) ||
            musicList[0].album?.toLowerCase().includes(search) ||
            musicList[0].albumArtist?.toLowerCase().includes(search)
        );
    });
});

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

    const data: MusicData[][] = [];
    for (const key of Object.keys(albumsMap).sort()) {
        data.push(MusicController.sortMusicList(albumsMap[key]));
    }

    return data;
}

// function onMouseWheel(
// 	e: WheelEvent & {
// 		currentTarget: EventTarget & HTMLDivElement;
// 	},
// ) {
// 	if (e.deltaX == -0) {
// 		e.preventDefault();
// 		element.scrollLeft += e.deltaY;
// 	}
// }

let unlistenMusicList: Unsubscriber
onMount(() => {
    updateItemWidth();
	MusicController.setMusicAlbumList(groupByAlbum());
	unlistenMusicList = musicList.subscribe(() => MusicController.setMusicAlbumList(groupByAlbum()));
});

onDestroy(() => {
    unlistenMusicList();
});

$effect(() => {
    itemHeight = itemElementHeight > itemHeight ? itemElementHeight : itemHeight;
    $swipeMinimumTop = itemHeight + paddingTop;
});
</script>

<svelte:window onresize={updateSize} />
<!--<div-->
<!--    class="grid auto-cols-[50%] sm:auto-cols-[33.3334%]-->
<!--        md-mdpi:auto-cols-[20%] lg-mdpi:auto-cols-[16.6667%] xl-mdpi:auto-cols-[12.5%]-->
<!--        md-hdpi:auto-cols-[20%] lg-hdpi:auto-cols-[16.6667%]-->
<!--        grid-rows-[1fr] w-full overflow-x-auto scrollbar-hidden"-->
<!--    style="padding-top: ${(isMobile() ? $mobileStatusBarHeight : 0) + 44}px"-->
<!--    bind:offsetHeight={elementHeight}-->
<!--	bind:this={element}-->
<!--    onwheel={onMouseWheel}>-->
<!--    {#each $musicAlbumList as musicList}-->
<!--        <AlbumItem {musicList} />-->
<!--    {/each}-->
<!--</div>-->

<div style="padding-top: {paddingTop}px;
    width: 100%;
    height: {itemHeight + paddingTop}px;">
    <VList
       class="scrollbar-hidden"
       {data}
       horizontal>
        {#snippet children(musicList, index)}
            {#if index === 0}
                <div class="h-fit" style="width: {itemWidth}px;"
                    bind:clientHeight={itemElementHeight}>
                    <AlbumItem {musicList} />
                </div>
            {:else}
                <div class="h-fit" style="width: {itemWidth}px;">
                    <AlbumItem {musicList} />
                </div>
            {/if}
        {/snippet}
    </VList>
</div>