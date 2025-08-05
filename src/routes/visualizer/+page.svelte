<script lang="ts">
import { onDestroy, onMount } from "svelte";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import View from "$lib/visualizers/vissonance/View";
import Iris from "$lib/visualizers/vissonance/visualizers/Iris";
import ToastController from "$lib/controllers/ToastController";
import MusicController from "$lib/controllers/MusicController";
import PageController from "$lib/controllers/PageController";
import { musicCurrentIndex } from "$lib/stores/music";
import type { Unsubscriber } from "svelte/store";
import type { MusicData } from "$lib/home/music/types";
import { isMobile } from "$lib/platform";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import Fracture from "$lib/visualizers/vissonance/visualizers/Fracture";
import Barred from "$lib/visualizers/vissonance/visualizers/Barred";
import HillFog from "$lib/visualizers/vissonance/visualizers/HillFog";
import Silk from "$lib/visualizers/vissonance/visualizers/Silk";
import Siphon from "$lib/visualizers/vissonance/visualizers/Siphon";

let marginTop = $derived((isMobile() ? $mobileStatusBarHeight : 0) + 40);

let container: HTMLDivElement;

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

	const visualizer = new Siphon();
	await visualizer.make();

	await setAudio();

	View.data.renderVisualization = visualizer.render.bind(visualizer);
}

async function setAudio(music: MusicData | null = null) {
	if (!MusicController.isPlaying()) return;

	try {
		const buffer = await MusicController.getBuffer(
			music ? music.path : MusicController.currentMusic().path,
		);
		if (buffer === null) return;
		await AudioAnalyser.makeAudio(new Uint8Array(buffer).buffer);
	} catch (e) {}
}

function onKeyDown(e: KeyboardEvent) {
	if (e.key === "Escape") PageController.back();
}

function toastError() {
	ToastController.error("Your OS WebView does not support WebGL.");
}

let unlistenMusicCurrentIndex: Unsubscriber;
onMount(() => {
	(async () => {
		await start();
		unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(async (index) =>
			setAudio(MusicController.getMusicByIndex(index)),
		);
	})();
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
<div id="actions" class="ms-3 font-light" style="margin-top: {marginTop}px;">
    <p class="text-opacity-background-80 text-3xl">Vissonance</p>
    <p class="text-opacity-background-60">by tariqksoliman</p>
</div>

<style lang="scss">
  //#actions {
  //  opacity: 0;
  //  transition: opacity 1s ease 3s;
  //
  //  &:hover {
  //    opacity: 1;
  //    transition-delay: 0s;
  //  }
  //}
</style>