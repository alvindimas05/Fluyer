<script lang="ts">
import "animate.css";
import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
import logHandler from "$lib/handlers/log";
import "../app.scss";
import TitleBar from "$lib/titlebar/TitleBar.svelte";
import { isMobile } from "$lib/platform";
    import MobileBackground from "$lib/backgrounds/MobileBackground.svelte";
interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();

logHandler();
</script>

<!-- FIXME: Layout is overflowing top and bottom on some android devices -->
<!-- TODO: Add option to enable AnimatedBackground on Mobile -->
{#if isMobile()}
    <MobileBackground/>
{:else}
    <AnimatedBackground/>
{/if}
<div class="w-screen h-screen fixed overflow-x-hidden scrollbar-hidden">
    {@render children?.()}
</div>
<TitleBar/>