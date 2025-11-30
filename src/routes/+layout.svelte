<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/features/animated_background/components/AnimatedBackground.svelte";
import "../app.scss";
import "toastify-js/src/toastify.css";
import { isDesktop, isLinux, isMobile, isWindows } from "$lib/platform";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";
import { PageRoutes } from "$lib/pages";
import { page } from "$app/state";
import SwipeGuide from "$lib/features/swipe_guide/components/SwipeGuide.svelte";
import DeveloperDebugOverlay from "$lib/features/developer_debug_overlay/components/DeveloperDebugOverlay.svelte";
import ToastService from "$lib/services/ToastService.svelte";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import mobileStore from "$lib/stores/mobile.svelte";
import musicStore from "$lib/stores/music.svelte";
import MusicPlayerService from "$lib/services/MusicPlayerService.svelte";
import UIInteractionService from "$lib/services/UIInteractionService.svelte";
import MobileService from "$lib/services/MobileService.svelte";
import settingStore from "$lib/stores/setting.svelte";
import FolderService from "$lib/services/FolderService.svelte";
import LogService from "$lib/services/LogService.svelte";
import Font from "$lib/ui/font/Font.svelte";
import FilterBar from "$lib/features/filterbar/components/FilterBar.svelte";
import TitleBar from "$lib/features/titlebar/components/TitleBar.svelte";

if (isWindows() || isLinux()) {
	import("$lib/scss/rounded-windows.scss");
}

if(isLinux()){
	import("$lib/scss/text-linux.scss");
}

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
let isAppReady = $state(false);

onMount(async () => {
    const now = performance.now();
    await Promise.all([
        LogService.initialize(),
        ToastService.initialize(),
        PersistentStoreService.initialize(),
        MusicPlayerService.initialize(),
        UIInteractionService.initialize(),
        MobileService.initialize(),
        FolderService.initialize(),
    ]);

    if (isDesktop()) {
        await getCurrentWindow().show();
        if(!await getCurrentWindow().isMaximized()) await getCurrentWindow().toggleMaximize();
    }

    isAppReady = true;

    console.log(`Front-end is initialized. Took ${performance.now() - now} ms`);
});
</script>

<Font />
{#if isAppReady}
    <AnimatedBackground />
{/if}
<div class="w-screen h-screen fixed scrollbar-hidden rounded-windows">
    {@render children?.()}
</div>
<div class="fixed top-0 left-0 w-full h-12 z-[99999] grid grid-cols-[1fr_auto]">
    {#if Array.isArray(musicStore.list)}
        {#if [PageRoutes.HOME, PageRoutes.HOME_PRODUCTION].includes(page.url.pathname)}
            <FilterBar />
        {/if}
        {#if isMobile() && mobileStore.showSwipeGuide}
            <SwipeGuide />
        {/if}
    {/if}
    {#if isDesktop()}
        <TitleBar />
    {/if}
</div>
{#if settingStore.developerMode}
    <DeveloperDebugOverlay />
{/if}