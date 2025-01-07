<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import logHandler from "$lib/handlers/log";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { invoke } from "@tauri-apps/api/core";
import { isDesktop, isMobile } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import { CommandsRoute } from "$lib/commands";
import HeadsetChange from "$lib/mobile/HeadsetChange.svelte";
interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
logHandler();

let statusBarHeight: number | null = $state(0);

async function getStatusBarHeight() {
	const barHeight = await invoke<number>(CommandsRoute.GET_STATUS_BAR_HEIGHT);
	statusBarHeight = barHeight > 28 ? 28 : barHeight;
}
if (isMobile()) {
	getStatusBarHeight();
	MusicController.listenSyncMusic();
}
MusicController.listenNextMusic();
</script>

<!-- TODO: Add option to enable AnimatedBackground on Mobile -->
<AnimatedBackground />
<div
    class={`w-screen h-screen fixed overflow-x-hidden scrollbar-hidden pt-6`}
    style={isMobile() ? `padding-top: ${statusBarHeight}px` : ""}
>
    {@render children?.()}
</div>
{#if isMobile()}
    <HeadsetChange/>
{/if}
<TitleBar />
