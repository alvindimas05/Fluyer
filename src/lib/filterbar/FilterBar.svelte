<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { isMacos, isMobile } from "$lib/platform";
import {filterAlbum, filterSearch} from "$lib/stores/filter";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import {onMount} from "svelte";
import {musicListType} from "$lib/stores/music";
import {MusicListType} from "$lib/home/music/types";
import {folderCurrent} from "$lib/stores/folder";
import FolderController from "$lib/controllers/FolderController";

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

let gridSize = $state("");
let listType = $derived($musicListType);

function updateGridSizing() {
    const w = window.innerWidth;
    const dpi = window.devicePixelRatio;

    for (const [minW, minDppx, width] of rules) {
        if (w >= minW && dpi >= minDppx) {
            const columnPercentage = width * window.innerWidth;
            // 1:2 ratio
            gridSize = `${columnPercentage}px ${columnPercentage * 2}px`;
            return;
        }
    }
    gridSize = "";
}

function toggleMusicListType() {
    $musicListType = listType === MusicListType.All ? MusicListType.Folder : MusicListType.All;
    if($musicListType === MusicListType.Folder){
        $filterAlbum = null;
        $folderCurrent = null;
        FolderController.setFolderList();
    }
}

onMount(() => {
    updateGridSizing();
});
</script>

<svelte:window onresize={updateGridSizing} />
<div class="w-full sm:w-fit h-6 sm:h-6 absolute top-0 grid gap-y-2 px-3 sm:px-0
    {isMacos() ? 'right-0' : 'left-0'}"
     style="margin-top: {isMobile() ? $mobileStatusBarHeight : 8}px;
        grid-template-columns: {gridSize};">
    <button class="w-full sm:w-auto h-fit sm:h-full bg-white/20 backdrop-blur-md text-white rounded-md shadow-md sm:mx-3 text-start
        animate__animated animate__fadeIn animate__slow"
        onclick={toggleMusicListType}>
        <div class="w-full h-full grid grid-cols-[min-content_auto] gap-x-2 px-3 py-1
            text-sm text-white/70">
            <div class="w-4">
                <Icon type={listType === MusicListType.Folder ? IconType.Note : IconType.Folder} />
            </div>
            <p>Browse {$musicListType === MusicListType.Folder ? 'All' : 'Folder'}</p>
        </div>
    </button>
    <div class="w-full sm:w-auto h-fit sm:h-full bg-white/20 backdrop-blur-md text-white rounded-md shadow-md sm:mx-3 order-first sm:order-last
        animate__animated animate__fadeIn animate__slow">
        <div class="w-full grid grid-cols-[auto_min-content] cursor-text px-3 py-1">
            <input
                    class="w-full bg-transparent placeholder:text-white/70 text-white outline-none text-sm"
                    placeholder="Search..."
                    bind:value={$filterSearch}
            />
            <div class="w-4">
                <Icon type={IconType.Search} />
            </div>
        </div>
    </div>
</div>
