<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { isAndroid, isDesktop, isMacos, isWindows } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import HeadsetChange from "$lib/mobile/HeadsetChange.svelte";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";
import Font from "$lib/font/Font.svelte";
import UIController from "$lib/controllers/UIController";
import MobileController from "$lib/controllers/MobileController";
import logHandler from "$lib/handlers/log";
import { goto } from "$app/navigation";
import { pageGotoRoute, pageGotoShow } from "$lib/stores/page";
import { PageRoutes } from "$lib/pages";

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
let isAppReady = $state(false);

async function initialize() {
	if (isDesktop()) await getCurrentWindow().show();
	if (isWindows()) await getCurrentWindow().toggleMaximize();

	logHandler();
	MusicController.initialize();
	UIController.initialize();
	MobileController.initialize();
	isAppReady = true;
}

// FIXME: When PlayerBar animation is still running. It will redirect instantly and not waiting for fadeIn effect.
let pageGotoShowCounter = 0;
async function onLayoutAnimationEnd() {
	if ($pageGotoShow) return;
	if (pageGotoShowCounter < 1 && location.pathname === PageRoutes.HOME) {
		pageGotoShowCounter = 1;
		return;
	}
	await goto($pageGotoRoute!);
	$pageGotoShow = true;
	pageGotoShowCounter = 0;
}

if (isWindows()) {
	onMount(async () => {
		setTimeout(() => {
			initialize();
		}, 1000);
	});
} else {
	initialize();
}
</script>

<Font />
<!-- TODO: Add option to enable AnimatedBackground on Mobile -->
{#if isAppReady}
    <AnimatedBackground />
{/if}
<!-- FIXME: Smooth Redirect doesn't work on Chromium browsers -->
<div
    class={`w-screen h-screen fixed scrollbar-hidden ` +
        (isMacos() &&
            `animate__animated ${$pageGotoShow ? "animate__fadeIn" : "animate__fadeOut"}`)}
    onanimationend={onLayoutAnimationEnd}
>
    {@render children?.()}
</div>
{#if isAndroid()}
    <HeadsetChange />
{/if}
{#if isDesktop()}
    <TitleBar />
{/if}
