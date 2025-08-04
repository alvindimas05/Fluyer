<script lang="ts">
    import {onDestroy, onMount} from "svelte";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import View from "$lib/visualizers/vissonance/View";
import Iris from "$lib/visualizers/vissonance/visualizers/Iris";
import ToastController from "$lib/controllers/ToastController";
import MusicController from "$lib/controllers/MusicController";
import PageController from "$lib/controllers/PageController";
import {musicCurrentIndex} from "$lib/stores/music";
import type {Unsubscriber} from "svelte/store";
    import type {MusicData} from "$lib/home/music/types";

let container: HTMLDivElement;

async function start(){
    try {
        const canvas = document.createElement('canvas');
        if(!(window.WebGLRenderingContext && (canvas.getContext('webgl') || canvas.getContext('experimental-webgl')))){
            toastError();
            return;
        }
    } catch (e) {
        toastError();
        return;
    }

    AudioAnalyser.initialize();
    View.initialize(container);

    await Iris.make();

    await setAudio();

    View.data.renderVisualization = Iris.render;
}

async function setAudio(music: MusicData | null = null){
    if(!MusicController.isPlaying()) return;

    try {
        const buffer = await MusicController.getBuffer(music ? music.path : MusicController.currentMusic().path);
        if(buffer === null) return;
        await AudioAnalyser.makeAudio(new Uint8Array(buffer).buffer);
    } catch (e) {}
}

function onKeyDown(e: KeyboardEvent){
    if(e.key === "Escape") PageController.back();
}

function toastError(){
    ToastController.error("Your OS WebView does not support WebGL.");
}

let unlistenMusicCurrentIndex: Unsubscriber;
onMount(() => {
    (async () => {
        await start();
        unlistenMusicCurrentIndex = musicCurrentIndex
            .subscribe(async (index) => setAudio(MusicController.getMusicByIndex(index)));
    })();
});

onDestroy(() => {
    unlistenMusicCurrentIndex();
});
</script>

<svelte:window
    onresize={View.onWindowResize}
    onkeydown={onKeyDown}/>
<div class="fixed w-full h-full z-[-1] bg-black"></div>
<div bind:this={container}></div>