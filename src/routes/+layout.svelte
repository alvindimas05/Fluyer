<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import "../app.scss";
import "toastify-js/src/toastify.css";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { isDesktop, isMobile, isWindows } from "$lib/platform";
import MusicController from "$lib/controllers/MusicController";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";
import Font from "$lib/font/Font.svelte";
import UIController from "$lib/controllers/UIController";
import MobileController from "$lib/controllers/MobileController.js";
import logHandler from "$lib/handlers/log";
import { PageRoutes } from "$lib/pages";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import FilterBar from "$lib/filterbar/FilterBar.svelte";
import { page } from "$app/state";
import LoadingController from "$lib/controllers/LoadingController";
import { loadingShow } from "$lib/stores/loading";
import { musicList } from "$lib/stores/music";
import SwipeGuide from "$lib/mobile/SwipeGuide.svelte";
import { mobileShowSwipeGuide } from "$lib/stores/mobile";
import FolderController from "$lib/controllers/FolderController";

if (isWindows()) {
	import("$lib/scss/rounded-windows.scss");
}

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
let isAppReady = $state(false);

async function initialize() {
	logHandler();
	await Promise.all([
		PersistentStoreController.initialize(),
		MusicController.initialize(),
		UIController.initialize(),
		MobileController.initialize(),
        FolderController.initialize(),
	]);

	if (isDesktop()){
        await getCurrentWindow().show();
        await getCurrentWindow().toggleMaximize();
    }

	LoadingController.listen();
	isAppReady = true;
}

onMount(initialize);
</script>

<Font />
{#if isAppReady}
    <AnimatedBackground />
{/if}
<div class="w-screen h-screen fixed scrollbar-hidden rounded-windows">
    {@render children?.()}
</div>
<div class="fixed top-0 left-0 w-full h-12 z-[99999] grid grid-cols-[1fr_auto]">
    {#if $loadingShow && Array.isArray($musicList)}
        {#if [PageRoutes.HOME, PageRoutes.HOME_PRODUCTION].includes(page.url.pathname)}
            <FilterBar />
        {/if}
        {#if isMobile() && $mobileShowSwipeGuide}
            <SwipeGuide />
        {/if}
    {/if}
    {#if isDesktop()}
        <TitleBar />
    {/if}
</div>

<svg style="display: none">
    <filter
            id="glass-distortion"
            x="0%"
            y="0%"
            width="100%"
            height="100%"
            filterUnits="objectBoundingBox"
    >
        <feTurbulence
                type="fractalNoise"
                baseFrequency="0.01 0.01"
                numOctaves="1"
                seed="5"
                result="turbulence"
        />
        <!-- Seeds: 14, 17,  -->

        <feComponentTransfer in="turbulence" result="mapped">
            <feFuncR type="gamma" amplitude="1" exponent="10" offset="0.5" />
            <feFuncG type="gamma" amplitude="0" exponent="1" offset="0" />
            <feFuncB type="gamma" amplitude="0" exponent="1" offset="0.5" />
        </feComponentTransfer>

        <feGaussianBlur in="turbulence" stdDeviation="3" result="softMap" />

        <feSpecularLighting
                in="softMap"
                surfaceScale="5"
                specularConstant="1"
                specularExponent="100"
                lighting-color="white"
                result="specLight"
        >
            <fePointLight x="-200" y="-200" z="300" />
        </feSpecularLighting>

        <feComposite
                in="specLight"
                operator="arithmetic"
                k1="0"
                k2="1"
                k3="1"
                k4="0"
                result="litImage"
        />

        <feDisplacementMap
                in="SourceGraphic"
                in2="softMap"
                scale="150"
                xChannelSelector="R"
                yChannelSelector="G"
        />
    </filter>
</svg>
