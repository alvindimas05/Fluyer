<script lang="ts">
import MusicController from "$lib/controllers/MusicController";
import { musicCurrent, musicsNext } from "$lib/stores/music";
import PlaylistItem from "./PlaylistItem.svelte";

let isMouseInsideArea = $state(false);
let animationClass = $state("");

function onMouseMove(
	e: MouseEvent & {
		currentTarget: EventTarget & Document;
	},
) {
	if (
		e.clientX > window.innerWidth - 20 &&
		e.clientY <= window.innerHeight - 8 * 16 &&
		!isMouseInsideArea
	) {
		isMouseInsideArea = true;
		animationClass = "animate__fadeInRight";
	}
}

function onMouseLeave(
	e: MouseEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	if (!isMouseInsideArea) return;
	animationClass = "animate__fadeOutRight";
}

function onAnimationEnd() {
	if (animationClass != "animate__fadeOutRight") return;
	isMouseInsideArea = false;
}
</script>

<svelte:document onmousemove={onMouseMove} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class={`fixed right-0 top-0 z-10 h-[calc(100%-8rem)] w-[25%] pe-3 pt-8
    ${isMouseInsideArea ? "" : "hidden"}`}
    onmouseleave={onMouseLeave}
>
    <div
        class={`bg-gray-700 bg-opacity-30 backdrop-blur-md rounded
    text-white h-full w-full p-3 animate__animated animate__faster
    ${animationClass}`}
        onanimationend={onAnimationEnd}
    >
        <div class="border border-gray-400 h-full w-full rounded flex flex-col">
            <p class="text-[1.5rem] font-semibold p-3">Playlist</p>
            <div class="flex-1 w-full overflow-auto scrollbar-hidden">
                {#if $musicCurrent}
                    <PlaylistItem music={$musicCurrent} isPlaying={true}/>
                {/if}
                {#each $musicsNext as music}
                    <PlaylistItem {music}/>
                {/each}
            </div>
        </div>
    </div>
</div>
