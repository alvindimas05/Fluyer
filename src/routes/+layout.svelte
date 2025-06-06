<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { isDesktop, isMacos, isWindows } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";
import Font from "$lib/font/Font.svelte";
import UIController from "$lib/controllers/UIController";
import MobileController from "$lib/controllers/MobileController";
import logHandler from "$lib/handlers/log";
import { goto } from "$app/navigation";
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
{#if isAppReady}
    <AnimatedBackground />
{/if}
<div
    class="w-screen h-screen fixed scrollbar-hidden">
    {@render children?.()}
</div>
{#if isDesktop()}
    <TitleBar />
{/if}
