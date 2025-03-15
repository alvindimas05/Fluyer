<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { isAndroid, isDesktop, isWindows } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import HeadsetChange from "$lib/mobile/HeadsetChange.svelte";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
let isAppReady = $state(false);

async function initialize(){
    if(isDesktop()) await getCurrentWindow().show();
    MusicController.handleInitialize();
    isAppReady = true;
}

if(isWindows()){
    onMount(() => {
        setTimeout(() => {
            getCurrentWindow().toggleMaximize();
            initialize();
        }, 1000);
    });
} else {
    initialize();
}
</script>

<!-- TODO: Add option to enable AnimatedBackground on Mobile -->
{#if isAppReady}
    <AnimatedBackground />
{/if}
<div class="w-screen h-screen fixed scrollbar-hidden">
    {@render children?.()}
</div>
{#if isAndroid()}
    <HeadsetChange/>
{/if}
{#if isDesktop()}
    <TitleBar />
{/if}