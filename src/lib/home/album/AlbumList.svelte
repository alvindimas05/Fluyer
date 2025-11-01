<script lang="ts">
import type { MusicData } from "$lib/home/music/types";
import { musicAlbumList, musicList } from "$lib/stores/music";
import AlbumItem from "./AlbumItem.svelte";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import { onDestroy, onMount } from "svelte";
import { isDesktop, isLinux, isMobile } from "$lib/platform";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import { swipeMinimumTop } from "$lib/stores";
import type { Unsubscriber } from "svelte/store";
import { VList } from "virtua/svelte";
import { filterAlbum, filterSearch } from "$lib/stores/filter";
import { filterBarHeight, filterBarSortAsc } from "$lib/stores/filterbar";
import sleep from "sleep-promise";

const rules = [
	// xhdpi (DPR > 2.0)
	[1536, 2.01, 0.125], // 2xl → 12.5%
	[1280, 2.01, 0.16667], // xl-xhdpi → 16.6667%
	[1024, 2.01, 0.2], // lg-xhdpi → 20%
	[768, 2.01, 0.25], // md-xhdpi → 25%
	[640, 2.01, 0.33334], // sm-xhdpi → 33.3334%

	// hdpi (1.01 ≤ DPR ≤ 2.0)
	[1536, 1.01, 0.125], // 2xl → 12.5%
	[1280, 1.01, 0.16667], // xl-hdpi → 16.6667%
	[1024, 1.01, 0.2], // lg-hdpi → 20%
	[768, 1.01, 0.25], // md-hdpi → 25%
	[640, 1.01, 0.33334], // sm-hdpi → 33.3334%

	// default (DPR <= 1.0)
	[1536, 0, 0.125], // 2xl → 12.5%
	[1280, 0, 0.16667], // xl → 16.6667%
	[1024, 0, 0.2], // lg → 20%
	[768, 0, 0.25], // md → 25%
	[640, 0, 0.33334], // sm → 33.3334%
];
let itemWidth = $state(0.5 * window.innerWidth);
let paddingTop = $derived(
	(isMobile() ? $mobileStatusBarHeight : 0) + $filterBarHeight,
);
let itemHeight = $derived(itemWidth + 52);

function updateSize() {
	updateItemWidth();
}

function updateItemWidth() {
	const w = window.innerWidth;
	const dpi = window.devicePixelRatio;

	for (const [minW, minDppx, width] of rules) {
		if (w >= minW && dpi >= minDppx) {
			itemWidth = width * window.innerWidth;
			return;
		}
	}
	itemWidth = 0.5 * window.innerWidth;
}

let data = $derived.by(() => {
	if (!Array.isArray($musicAlbumList)) return [];

	const sortAsc = $filterBarSortAsc;

	const search = $filterSearch.toLowerCase();
	let list = $musicAlbumList;
	if (!search) return sortAsc ? list : list.toReversed();

	list = list.filter((musicList) => {
		return (
			($filterAlbum && musicList[0].album === $filterAlbum.name) ||
			musicList[0].album?.toLowerCase().includes(search) ||
			musicList[0].albumArtist?.toLowerCase().includes(search)
		);
	});
	if (!sortAsc) list.reverse();
	return list;
});

function groupByAlbum(): MusicData[][] {
	let musicList = MusicController.musicList()!;
	const albumsMap = musicList.reduce(
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

function onMouseWheel(
	e: WheelEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	if (e.deltaX == -0) {
		e.preventDefault();
		e.currentTarget.scrollLeft += e.deltaY;
	}
}

let unlistenMusicList: Unsubscriber;
onMount(() => {
	updateSize();
	MusicController.setMusicAlbumList(groupByAlbum());
	unlistenMusicList = musicList.subscribe(() =>
		MusicController.setMusicAlbumList(groupByAlbum()),
	);
});

onDestroy(() => {
	unlistenMusicList();
});

$effect(() => {
	$swipeMinimumTop = paddingTop + itemHeight;
});
</script>

<svelte:window onresize={updateSize} />

<div class="w-full" style="height: {itemHeight}px;">
    <!-- Note: Ignore this  -->
    <VList onwheel={onMouseWheel}
           class="scrollbar-hidden overflow-y-clip"
           {data}
           horizontal>
        {#snippet children(musicList, index)}
            <div style="width: {itemWidth}px;">
                <AlbumItem {musicList} />
            </div>
        {/snippet}
    </VList>
</div>