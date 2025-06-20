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
    import { PageRoutes } from "$lib/pages";
    import PersistentStoreController from "$lib/controllers/PersistentStoreController";
    import FilterBar from "$lib/filterbar/FilterBar.svelte";
    import { page } from "$app/state";
    import LoadingController from "$lib/controllers/LoadingController";
    import { loadingShow } from "$lib/stores/loading";
    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();
    let isAppReady = $state(false);

    async function initialize() {
        if (isDesktop()) await getCurrentWindow().show();
        if (isWindows()) await getCurrentWindow().toggleMaximize();

        logHandler();
        await Promise.all([
            PersistentStoreController.initialize(),
            MusicController.initialize(),
            UIController.initialize(),
            MobileController.initialize(),
        ]);
        LoadingController.listen();
        isAppReady = true;
    }

    onMount(() => setTimeout(initialize, isWindows() ? 1000 : 0));
</script>

<Font />
{#if isAppReady}
    <AnimatedBackground />
{/if}
<div class="w-screen h-screen fixed scrollbar-hidden">
    {@render children?.()}
</div>
<div class="fixed top-0 left-0 w-full h-12 z-[99999] grid grid-cols-[1fr_auto]">
    {#if $loadingShow && page.url.pathname === PageRoutes.HOME}
        <FilterBar />
    {/if}
    {#if isDesktop()}
        <TitleBar />
    {/if}
</div>
