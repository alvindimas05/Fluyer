import {MusicListType} from "$lib/features/music/types";
import musicStore from "$lib/stores/music.svelte";
import filterStore from "$lib/stores/filter.svelte";
import folderStore from "$lib/stores/folder.svelte";
import LibraryService from "$lib/services/LibraryService.svelte";
import FolderService from "$lib/services/FolderService.svelte";

const RESPONSIVE_RULES = [
    [1280, 2.01, 4], [1024, 2.01, 3], [768, 2.01, 2],
    [1536, 1.01, 4], [1280, 1.01, 3], [768, 1.01, 2],
    [1536, 1.0, 4],  [1024, 1.0, 3],  [768, 1.0, 2],
];

const state = $state({
    columnCount: 1,
});

function updateColumnCount() {
    const w = window.innerWidth;
    const dpi = window.devicePixelRatio;

    for (const [minW, minDppx, cols] of RESPONSIVE_RULES) {
        if (w >= minW && dpi >= minDppx) {
            state.columnCount = cols;
            return;
        }
    }
    state.columnCount = 1;
}

const updateSize = () => updateColumnCount();

const data = $derived.by(() => {
    if (!Array.isArray(musicStore.list)) return [];

    const search = filterStore.search.toLowerCase();
    const album = filterStore.album;
    const hasSearch = search.length > 0;
    const hasAlbum = !!album;
    const isFolderMode = musicStore.listType === MusicListType.Folder;

    const filteredMusic = LibraryService.sortMusicList(
        musicStore.list.filter(music => {
            const matchesSearch = hasSearch &&
                [
                    music.album, music.title,
                    music.artist, music.albumArtist
                ].some(v => v?.toLowerCase().includes(search));

            const matchesAlbum = hasAlbum && album.name === music.album;
            const matchesFolder = FolderService.containsMusic(
                music,
                folderStore.currentFolder
            );

            if (isFolderMode)
                return matchesFolder && (!hasSearch || matchesSearch);

            if (!hasAlbum)
                return !hasSearch || matchesSearch;

            return matchesAlbum && (!hasSearch || matchesSearch);
        })
    );

    let filteredFolders = folderStore.list.filter(f =>
        f.path.toLowerCase().includes(search)
    );

    if (!filterStore.bar.sortAsc) filteredFolders = [...filteredFolders].reverse();

    let finalList: any[] = filterStore.album
        ? LibraryService.sortMusicList(filteredMusic)
        : [...filteredMusic];

    if (!filterStore.bar.sortAsc) finalList.reverse();

    if (isFolderMode) {
        const nonEmpty = filteredFolders.filter(
            f => FolderService.getMusicList(f).length > 0
        );
        finalList.push(...nonEmpty);
    }

    return finalList;
});

export function useMusicList() {
    $effect(() => {
        musicStore.listType;
        updateSize();
    });

    return {
        state,

        get data() {
            return data;
        },

        updateSize,
    };
}