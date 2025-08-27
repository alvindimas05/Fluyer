<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { isMacos, isMobile } from "$lib/platform";
import {filterSearch} from "$lib/stores/filter";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import {onMount} from "svelte";
import {musicListType} from "$lib/stores/music";
import {MusicListType} from "$lib/home/music/types";
import UIController from "$lib/controllers/UIController";
import Glass from "$lib/glass/Glass.svelte";
import {filterBarHeight} from "$lib/stores/filterbar";

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

let element: HTMLDivElement;
let gridSize = $state("");

function updateGridSizing() {
    const w = window.innerWidth;
    const dpi = window.devicePixelRatio;

    for (const [minW, minDppx, width] of rules) {
        if (w >= minW && dpi >= minDppx) {
            const columnPercentage = width * window.innerWidth;
            // 1:2 ratio
            gridSize = isMacos() ? `${columnPercentage}px ${columnPercentage * 2}px` : `${columnPercentage * 2}px ${columnPercentage}px`;
            return;
        }
    }
    gridSize = "";
}

function updateFilterBarHeight(){
    $filterBarHeight = element.offsetHeight + 8;
}

function updateSize(){
    updateGridSizing();
    updateFilterBarHeight()
}
onMount(() => {
    updateSize();
    setTimeout(updateFilterBarHeight, 0);
});
</script>

<svelte:window onresize={updateSize} />
<div class="w-full sm:grid gap-y-2 px-3 sm:px-0 pb-3 pointer-events-none
    {isMacos() ? 'justify-end' : ''}
    {isMacos() ? 'right-0' : 'left-0'}"
    style="margin-top: {isMobile() ? $mobileStatusBarHeight : 8}px;
        grid-template-columns: {gridSize};"
    bind:this={element}>
    <Glass class="!hidden sm:!block h-fit sm:h-full sm:mx-3 text-start pointer-events-auto
        cursor-pointer hover:bg-white/10
        animate__animated animate__fadeIn animate__slow"
       padding="4px"
       paddingHover="6px"
        events={{
            onclick: UIController.toggleMusicListType
        }}>
        <div class="w-full h-full grid grid-cols-[min-content_auto] items-center gap-x-2 px-3
            text-sm text-white/70">
            <div class="w-4">
                <Icon type={$musicListType === MusicListType.Folder ? IconType.Note : IconType.Folder} />
            </div>
            <p>Browse {$musicListType === MusicListType.Folder ? 'All' : 'Folder'}</p>
        </div>
    </Glass>
    <Glass class="h-fit sm:h-full pointer-events-auto p-0 sm:mx-3
        {isMacos() ? 'order-last' : 'order-first'}
        animate__animated animate__fadeIn animate__slow"
        padding="6px"
        paddingHover="8px">
        <div class="w-full h-full grid grid-cols-[auto_min-content] items-center cursor-text px-2">
            <input
                    class="w-full h-full bg-transparent placeholder:text-white/70 text-white outline-none text-sm"
                    placeholder="Search..."
                    bind:value={$filterSearch}
            />
            <div class="w-4">
                <Icon type={IconType.Search} />
            </div>
        </div>
    </Glass>
</div>