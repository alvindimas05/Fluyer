<script lang="ts">
import {onMount} from "svelte";
import AudioAnalyser from "$lib/visualizers/vissonance/AudioAnalyser";
import View from "$lib/visualizers/vissonance/View";
import Iris from "$lib/visualizers/vissonance/visualizers/Iris";
import ToastController from "$lib/controllers/ToastController";

let container: HTMLDivElement;

async function test(){
    try {
        const canvas = document.createElement('canvas');
        if(!(window.WebGLRenderingContext && (canvas.getContext('webgl') || canvas.getContext('experimental-webgl')))){
            ToastController.error("Your OS WebView does not support WebGL.");
            return;
        }
    } catch (e) {
        ToastController.error("Your OS WebView does not support WebGL.");
        return;
    }

    AudioAnalyser.initialize();
    View.initialize(container);

    Iris.make();
    View.data.renderVisualization = Iris.render;

    const request = new XMLHttpRequest();
    request.open("GET", "https://tariqksoliman.github.io/Vissonance/songs/Sex Whales & Roee Yeger - Where Was I (ft. Ashley Apollodor).mp3");
    request.responseType = "arraybuffer";
    request.onload = () => {
        AudioAnalyser.makeAudio(request.response);
    };
    request.send();
}
onMount(test);
</script>

<svelte:window onresize={View.onWindowResize} />
<div class="fixed w-full h-full z-[-1] bg-black"></div>
<div bind:this={container}></div>