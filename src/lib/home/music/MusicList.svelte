<script lang="ts">
import { musicList, musicListType } from "$lib/stores/music";
import MusicItem from "./MusicItem.svelte";
import { VList } from "virtua/svelte";
import { onDestroy, onMount } from "svelte";
import { filterAlbum, filterSearch } from "$lib/stores/filter";
import MusicController from "$lib/controllers/MusicController";
import { folderCurrent, folderList } from "$lib/stores/folder";
import { MusicListType } from "$lib/home/music/types";
import FolderController from "$lib/controllers/FolderController";
import { playerBarHeight } from "$lib/stores/playerbar";
import { filterBarSortAsc } from "$lib/stores/filterbar";

// [minWidth, minDppx, columns]
const rules = [
    [1280, 2.01, 4], [1024, 2.01, 3], [768, 2.01, 2],
    [1536, 1.01, 4], [1280, 1.01, 3], [768, 1.01, 2],
    [1536, 1.0, 4],  [1024, 1.0, 3],  [768, 1.0, 2],
];

let columnCount = $state(1);

function updateColumnCount() {
    const w = window.innerWidth;
    const dpi = window.devicePixelRatio;

    for (const [minW, minDppx, cols] of rules) {
        if (w >= minW && dpi >= minDppx) {
            columnCount = cols;
            return;
        }
    }
    columnCount = 1;
}

const updateSize = () => updateColumnCount();

let data = $derived.by(() => {
    if (!Array.isArray($musicList)) return [];

    const search = $filterSearch.toLowerCase();
    const album = $filterAlbum;
    const hasSearch = search.length > 0;
    const hasAlbum = !!album;
    const isFolderMode = $musicListType === MusicListType.Folder;

    // Music Filter
    const list = MusicController.sortMusicList(
        $musicList.filter((music) => {
            const matchesSearch =
                hasSearch &&
                [
                    music.album,
                    music.title,
                    music.artist,
                    music.albumArtist,
                ].some((v) => v?.toLowerCase().includes(search));

            const matchesAlbum = hasAlbum && album.name === music.album;
            const matchesFolder = FolderController.isMusicInFolder(
                music,
                $folderCurrent,
            );

            if (isFolderMode)
                return matchesFolder && (!hasSearch || matchesSearch);

            if (!hasAlbum)
                return !hasSearch || matchesSearch;

            return matchesAlbum && (!hasSearch || matchesSearch);
        })
    );

    // Folder Filter
    let filteredFolders = $folderList
        .filter((folder) =>
            folder.path.toLowerCase().includes(search)
        );

    if (!$filterBarSortAsc) filteredFolders = [...filteredFolders].reverse();

    // Sort again if Album Filter
    let finalList = $filterAlbum
        ? MusicController.sortMusicList(list)
        : [...list];

    if (!$filterBarSortAsc) finalList.reverse();

    // Paginate into rows
    const chunk = (arr: any[]) => {
        const rows: any[][] = [];
        for (let i = 0; i < arr.length; i += columnCount)
            rows.push(arr.slice(i, i + columnCount));
        return rows;
    };

    const result = chunk(finalList);

    // Add Folders (only in folder mode)
    if (isFolderMode) {
        const nonEmptyFolders = filteredFolders.filter(
            (f) => FolderController.getMusicListFromFolder(f).length > 0
        );

        result.push(...chunk(nonEmptyFolders));
    }

    return result;
});


const unsubscribeMusicListType = musicListType.subscribe(() =>
    setTimeout(updateSize)
);

onMount(updateSize);
onDestroy(unsubscribeMusicListType);
</script>

<svelte:window onresize={updateSize} />

<div class="h-full px-3">
    {#if data && columnCount}
        <VList
                class="scrollbar-hidden"
                {data}
                style="padding-bottom: {$playerBarHeight}px;"
                getKey={(_, i) => i}
        >
            {#snippet children(list)}
                <div
                        class="grid gap-x-6"
                        style="grid-template-columns: repeat({columnCount}, minmax(0, 1fr))"
                >
                    {#each list as item}
                        {#if 'duration' in item}
                            <MusicItem music={item} />
                        {:else}
                            <MusicItem folder={item} />
                        {/if}
                    {/each}
                </div>
            {/snippet}
        </VList>
    {/if}
</div>
