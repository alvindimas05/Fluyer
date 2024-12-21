<script lang="ts">
    import "animate.css";
    import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
    import logHandler from "$lib/handlers/log";
    import "../app.scss";
    import TitleBar from "$lib/titlebar/TitleBar.svelte";
    import { invoke } from "@tauri-apps/api/core";
    interface Props {
        children?: import("svelte").Snippet;
    }

    let { children }: Props = $props();
    logHandler();

    let statusBarHeight: number | null = $state(0);

    async function getStatusBarHeight() {
        statusBarHeight = await invoke<number>("get_status_bar_height");
    }
    getStatusBarHeight();
</script>

<!-- TODO: Add option to enable AnimatedBackground on Mobile -->
<AnimatedBackground />
<div
    class="w-screen h-screen fixed overflow-x-hidden scrollbar-hidden lg:pt-6"
    style={`padding-top: ${statusBarHeight}px`}
>
    {@render children?.()}
</div>
<TitleBar />
