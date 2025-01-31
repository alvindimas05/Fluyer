<script lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { isMobile } from "$lib/platform";

const appWindow = getCurrentWindow();
function onMouseDown(
	e: MouseEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	if (e.buttons === 1) {
		e.detail === 2 ? appWindow.toggleMaximize() : appWindow.startDragging();
	}
}
</script>

{#if !isMobile()}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="fixed top-0 left-0 w-full h-8 z-[99999]" onmousedown={onMouseDown}></div>
{/if}
