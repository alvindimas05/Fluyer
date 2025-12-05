import {IconThemeType, IconType} from "$lib/ui/icon/types";
import {isMacos} from "$lib/platform";
import {type FolderData, MusicListType} from "$lib/features/music/types";
import filterStore from "$lib/stores/filter.svelte";
import iconStore from "$lib/stores/icon.svelte";
import folderStore from "$lib/stores/folder.svelte";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import musicStore from "$lib/stores/music.svelte";

const RESPONSIVE_RULES = [
    [1536, 2.01, 0.125],
    [1280, 2.01, 0.16667],
    [1024, 2.01, 0.2],
    [768, 2.01, 0.25],
    [640, 2.01, 0.33334],
    [1536, 1.01, 0.125],
    [1280, 1.01, 0.16667],
    [1024, 1.01, 0.2],
    [768, 1.01, 0.25],
    [640, 1.01, 0.33334],
    [1536, 0, 0.125],
    [1280, 0, 0.16667],
    [1024, 0, 0.2],
    [768, 0, 0.25],
    [640, 0, 0.33334],
];


const musicListOptions = [
    { value: MusicListType.All, icon: IconType.MusicListTypeAll, label: "All" },
    { value: MusicListType.Album, icon: IconType.MusicListTypeAlbum, label: "Album" },
    { value: MusicListType.Music, icon: IconType.MusicListTypeMusic, label: "Music" },
    { value: MusicListType.Folder, icon: IconType.MusicListTypeFolder, label: "Folder" },
    { value: MusicListType.Playlist, icon: IconType.Unknown, label: "Playlist" },
];

let element: HTMLDivElement;
let state = $state({
    gridSize: '',
});

const iconSize = $derived.by(() => {
    switch (iconStore.theme){
        case IconThemeType.Phosphor: return 19;
        case IconThemeType.Material: return 18;
        case IconThemeType.Lucide: return 20;
    }
});

function updateGridSizing() {
    const w = window.innerWidth;
    const dpi = window.devicePixelRatio;

    for (const [minW, minDppx, width] of RESPONSIVE_RULES) {
        if (w >= minW && dpi >= minDppx) {
            const columnPercentage = width * window.innerWidth;
            state.gridSize = isMacos()
                ? `${columnPercentage}px ${columnPercentage * 2}px`
                : `${columnPercentage * 2}px ${columnPercentage}px`;
            return;
        }
    }
    state.gridSize = "";
}

function updateFilterBarHeight() {
    if (!element) return;
    filterStore.bar.height = element.offsetHeight + (window.innerWidth > 640 ? 8 : 16);
}

function toggleSort() {
    filterStore.bar.sortAsc = !filterStore.bar.sortAsc;
}

async function handleToggleChange(type: MusicListType){
    filterStore.album = null;
    folderStore.currentFolder = null;

    // Set the current folder to the first music path if only one is set
    if(type === MusicListType.Folder) {
        const musicPaths = await PersistentStoreService.musicPath.get();
        folderStore.currentFolder = musicPaths.length === 1 ? { path: musicPaths[0] } as FolderData : null;
    }

    musicStore.listType = type;

    console.log("Music list type changed to:", type);
}

function updateSize() {
    updateGridSizing();
    setTimeout(updateFilterBarHeight);
}

export function useFilterBar() {
    return {
        state,

        musicListOptions,
        get element() {
            return element;
        },
        set element(value) {
            element = value;
            updateSize();
        },
        get iconSize() {
            return iconSize;
        },
        toggleSort,
        handleToggleChange,
        updateSize,
    };
}