<script lang="ts">
import { onDestroy, onMount } from "svelte";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import View from "$lib/visualizers/vissonance/View";
import ToastController from "$lib/controllers/ToastController";
import MusicController from "$lib/controllers/MusicController";
import PageController from "$lib/controllers/PageController";
import { musicCurrentIndex } from "$lib/stores/music";
import type { MusicData } from "$lib/home/music/types";
import { isMobile } from "$lib/platform";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import Barred from "$lib/visualizers/vissonance/visualizers/Barred";
import Fracture from "$lib/visualizers/vissonance/visualizers/Fracture";
// import HillFog from "$lib/visualizers/vissonance/visualizers/HillFog";
import Iris from "$lib/visualizers/vissonance/visualizers/Iris";
import Silk from "$lib/visualizers/vissonance/visualizers/Silk";
import Siphon from "$lib/visualizers/vissonance/visualizers/Siphon";
import Tricentric from "$lib/visualizers/vissonance/visualizers/Tricentric";
import type Visualizer from "$lib/visualizers/vissonance/visualizers/Visualizer";
import { showThenFade } from "$lib/controllers/UIController";

let marginTop = $derived((isMobile() ? $mobileStatusBarHeight : 0) + 40);

let container: HTMLDivElement;

const DEFAULT_VISUALIZER_INDEX = 0;
let visualizers: Visualizer[] = [
	new Barred(),
	new Fracture(),
	// new HillFog(),
	new Iris(),
	new Silk(),
	new Tricentric(),
	new Siphon(),
];
let currentVisualizerIndex = $state(-1);

async function start() {
	try {
		const canvas = document.createElement("canvas");
		if (
			!(
				window.WebGLRenderingContext &&
				(canvas.getContext("webgl") || canvas.getContext("experimental-webgl"))
			)
		) {
			toastError();
			return;
		}
	} catch (e) {
		toastError();
		return;
	}

	AudioAnalyser.initialize();
	View.initialize(container);

	await setCurrentVisualizer(DEFAULT_VISUALIZER_INDEX);

	await setAudio();
}

async function setCurrentVisualizer(index: number) {
	if (index === currentVisualizerIndex) return;

	View.data.renderVisualization = null;
	if (visualizers[currentVisualizerIndex])
		visualizers[currentVisualizerIndex].destroy();
	currentVisualizerIndex = index;
	View.clear();
	await visualizers[currentVisualizerIndex].make();
	View.data.renderVisualization = visualizers[
		currentVisualizerIndex
	].render.bind(visualizers[currentVisualizerIndex]);
}

async function setAudio(music: MusicData | null = null) {
	if (!MusicController.isPlaying) return;

	try {
		let now = performance.now();
		const buffer = await MusicController.getVisualizerBuffer(
			music ? music.path : MusicController.currentMusic.path,
		);
		if (buffer === null) return;
		console.log(
			"MusicController.getVisualizerBuffer took",
			performance.now() - now,
			"ms",
		);

		now = performance.now();
		await AudioAnalyser.makeAudio(new Uint8Array(buffer).buffer);
		console.log("AudioAnalyser.makeAudio took", performance.now() - now, "ms");
	} catch (e) {}
}

function onKeyDown(e: KeyboardEvent) {
	if (e.key === "Escape") PageController.back();
}

function toastError() {
	ToastController.error("Your OS WebView does not support WebGL.");
}

let unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(async (index) => {
	if (currentVisualizerIndex === -1) return;
	visualizers[currentVisualizerIndex].executeOnNewSong();
	setAudio(MusicController.getMusicByIndex(index));
});
onMount(() => {
	start();
});

onDestroy(() => {
	if (unlistenMusicCurrentIndex) unlistenMusicCurrentIndex();
	try {
		View.destroy();
	} catch (e) {}
	try {
		AudioAnalyser.destroy();
	} catch (e) {}
});
</script>

<svelte:window
    onresize={View.onWindowResize}
    onkeydown={onKeyDown}/>
<div class="fixed w-full h-full z-[-2] bg-black"></div>
<div class="fixed w-full h-full z-[-1]" bind:this={container}></div>
<div class="ms-3 font-light w-fit show-then-fade" style="margin-top: {marginTop}px;"
    use:showThenFade>
    <p class="text-gray-300 text-3xl">Vissonance</p>
    <p class="text-gray-400">by tariqksoliman</p>
    <ul class="w-fit">
        {#each visualizers as visualizer, i}
            <li class="cursor-pointer hover:text-gray-300"
                class:text-gray-500={i !== currentVisualizerIndex}
                class:text-gray-300={i === currentVisualizerIndex}
                onclick={() => setCurrentVisualizer(i)}>
                {visualizer.name}
                {#if visualizer.name === "Siphon"}
                    (EPILEPSY WARNING)
                {/if}
            </li>
        {/each}
    </ul>
    <p class="mt-2 text-gray-400 hover:text-gray-300 cursor-pointer"
        onclick={() => PageController.back()}>
        Back</p>
</div>