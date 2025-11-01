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
import {filterBarHeight, filterBarSortAsc} from "$lib/stores/filterbar";
import Toggle from "$lib/components/Toggle.svelte";
import View from "$lib/components/View.svelte";
import Button from "$lib/components/Button.svelte";
import Input from "$lib/components/Input.svelte";

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
    $filterBarHeight = element.offsetHeight + (window.innerWidth > 640 ? 8 : 16);
}

function toggleSort(){
    $filterBarSortAsc = !$filterBarSortAsc;
}

function updateSize(){
    updateGridSizing();
    updateFilterBarHeight();
    setTimeout(updateFilterBarHeight, 0);
}
onMount(updateSize);
</script>

<svelte:window onresize={updateSize} />
<div class="w-full grid gap-y-2 px-3 sm:px-0 sm:pb-3 pointer-events-none
    {isMacos() ? 'justify-end' : ''}
    {isMacos() ? 'right-0' : 'left-0'}
    animate__animated animate__fadeIn animate__slow"
    style="margin-top: {isMobile() ? $mobileStatusBarHeight : 8}px;
        grid-template-columns: {gridSize};"
    bind:this={element}>
    <div class="grid pointer-events-none grid-cols-[auto_4rem] md:grid-cols-[auto_30%] sm:mx-3 gap-x-2 sm:gap-x-4 {isMacos() ? 'justify-end' : 'justify-start'}">
        <div>
            <Button class="h-full aspect-square rounded grid justify-center pointer-events-auto p-1 md:p-0"
                onclick={toggleSort}>
                <div class="w-5">
                    {#if $filterBarSortAsc}
                        <Icon type={IconType.SortAsc} />
                    {:else}
                        <Icon type={IconType.SortDesc} />
                    {/if}
                </div>
            </Button>
        </div>
        <Toggle class="w-full h-full pointer-events-auto"
            checked={$musicListType === MusicListType.Folder}
            checkedIcon={IconType.Folder}
            uncheckedIcon={IconType.Note}
            onchange={UIController.toggleMusicListType} />
    </div>
    <Input
        class="h-fit sm:h-full pointer-events-auto p-0 sm:mx-3
            {isMacos() ? 'order-last' : 'order-first'} rounded"
        icon={IconType.Search}
        placeholder="Search..."
        bind:value={$filterSearch}
    />
</div>