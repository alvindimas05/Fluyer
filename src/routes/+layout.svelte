<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import logHandler from "$lib/handlers/log";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { invoke } from "@tauri-apps/api/core";
import { isAndroid, isDesktop, isIos, isMobile } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import { CommandsRoute } from "$lib/commands";
import HeadsetChange from "$lib/mobile/HeadsetChange.svelte";

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
logHandler();

MusicController.listenSyncMusic();
MusicController.listenNextMusic();
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