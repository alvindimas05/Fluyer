<script lang="ts">
import {type MusicData, MusicListType} from "$lib/features/music/types";
import {onMount} from "svelte";
import {isMobile} from "$lib/platform";
import {type VirtualizerHandle, VList} from "virtua/svelte";
import mobileStore from "$lib/stores/mobile.svelte";
import filterStore from "$lib/stores/filter.svelte";
import musicStore from "$lib/stores/music.svelte";
import playerBarStore from "$lib/stores/playerbar.svelte";
import sidebarStore from "$lib/stores/sidebar.svelte";

const RESPONSIVE_RULES = [
    [1536, 2.01, 0.125], // xhdpi 2xl → 12.5%
    [1280, 2.01, 0.16667], // xl-xhdpi → 16.6667%
    [1024, 2.01, 0.2], // lg-xhdpi → 20%
    [768, 2.01, 0.25], // md-xhdpi → 25%
    [640, 2.01, 0.33334], // sm-xhdpi → 33.3334%
    [1536, 1.01, 0.125], // hdpi 2xl → 12.5%
    [1280, 1.01, 0.16667], // xl-hdpi → 16.6667%
    [1024, 1.01, 0.2], // lg-hdpi → 20%
    [768, 1.01, 0.25], // md-hdpi → 25%
    [640, 1.01, 0.33334], // sm-hdpi → 33.3334%
    [1536, 0, 0.125], // default 2xl → 12.5%
    [1280, 0, 0.16667], // xl → 16.6667%
    [1024, 0, 0.2], // lg → 20%
    [768, 0, 0.25], // md → 25%
    [640, 0, 0.33334], // sm → 33.3334%
];

let virtualizerHandle: VirtualizerHandle;
let columnCount = $state(2);
let itemWidth = $state(0.5 * window.innerWidth);

let paddingTop = $derived((isMobile() ? mobileStore.statusBarHeight : 0) + filterStore.bar.height);
let itemHeight = $derived(itemWidth + (window.innerWidth > 640 ? 52 : 44));
let isHorizontal = $derived(musicStore.listType !== MusicListType.Album);

function updateItemWidth() {
    const width = window.innerWidth;
    const dpr = window.devicePixelRatio;

    for (const [minWidth, minDppx, widthRatio] of RESPONSIVE_RULES) {
        if (width >= minWidth && dpr >= minDppx) {
            itemWidth = widthRatio * width;
            columnCount = Math.round(1 / widthRatio);
            return;
        }
    }

    columnCount = 2;
    itemWidth = 0.5 * width;
}

function chunkArray(array: MusicData[][], size: number) {
    const chunks: MusicData[][][] = [];
    for (let i = 0; i < array.length; i += size) {
        chunks.push(array.slice(i, i + size));
    }
    return chunks;
}

let data: any[] = $derived.by(() => {
    if (!Array.isArray(musicStore.albumList)) return [];

    const search = filterStore.search.toLowerCase();
    let list = musicStore.albumList;

    if (search || filterStore.album) {
        list = list.filter((musicList) => {
            const firstItem = musicList[0];
            return (
                (filterStore.album && firstItem.album === filterStore.album.name) ||
                firstItem.album?.toLowerCase().includes(search) ||
                firstItem.albumArtist?.toLowerCase().includes(search)
            );
        });
    }

    if (!filterStore.bar.sortAsc) list = list.toReversed();

    return musicStore.listType === MusicListType.Album
        ? chunkArray(list, columnCount)
        : list;
});

function onMouseWheel(e: WheelEvent & { currentTarget: EventTarget & HTMLDivElement }) {
    if (e.deltaX === 0) {
        e.preventDefault();
        e.currentTarget.scrollLeft += e.deltaY;
    }
}

function scrollTo(index: number) {
    virtualizerHandle.scrollToIndex(index, { align: 'nearest', smooth: true });
}

$effect(() => {
    sidebarStore.swipeMinimumTop = paddingTop + itemHeight;
});

$effect(() => {
    scrollTo(musicStore.albumListScrollIndex);
    musicStore.albumListScrollIndex = -1;
});

onMount(updateItemWidth);
</script>

<svelte:window onresize={updateItemWidth} />

<div class="w-full" style="height: {isHorizontal ? itemHeight : window.innerHeight}px;">
    {#key isHorizontal}
        <VList
                onwheel={isHorizontal ? onMouseWheel : undefined}
                {data}
                class="scrollbar-hidden {isHorizontal ? '' : 'overflow-y-clip'}"
                horizontal={isHorizontal}
                style="padding-bottom: {isHorizontal ? 0 : playerBarStore.height + filterStore.bar.height + mobileStore.navigationBarHeight + mobileStore.statusBarHeight}px;"
                getKey={(_, i) => i}
                bind:this={virtualizerHandle}>
            {#snippet children(dataList, index)}
                {#if isHorizontal}
                    <div style="width: {itemWidth}px;">
                        <AlbumItem musicList={dataList} {index} />
                    </div>
                {:else}
                    <div class="flex">
                        {#each dataList as musicList, dataIndex}
                            <div style="width: {itemWidth}px;">
                                <AlbumItem {musicList} index={(index * columnCount) + dataIndex} />
                            </div>
                        {/each}
                    </div>
                {/if}
            {/snippet}
        </VList>
    {/key}
</div>