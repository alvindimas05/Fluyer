<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { isAndroid, isDesktop } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import HeadsetChange from "$lib/mobile/HeadsetChange.svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
MusicController.handleInitialize();
getCurrentWindow().show();
</script>

<!-- TODO: Add option to enable AnimatedBackground on Mobile -->
<AnimatedBackground />
<div class="w-screen h-screen fixed scrollbar-hidden">
    {@render children?.()}
</div>
{#if isAndroid()}
    <HeadsetChange/>
{/if}
{#if isDesktop()}
    <TitleBar />
{/if}