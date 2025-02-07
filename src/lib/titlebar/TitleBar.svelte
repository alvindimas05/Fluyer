<script lang="ts">
import { isMacos, isWindows } from "$lib/platform";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

let isMaximized = $state(true);
const window = getCurrentWindow();
function onMouseDown(
	e: MouseEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	if (e.buttons === 1) {
		e.detail === 2 ? window.toggleMaximize() : window.startDragging();
	}
}

let snapOverlayTimer: ReturnType<typeof setTimeout> | null = null;
function showSnapOverlay() {
	window.setFocus().then(() => invoke("decorum_show_snap_overlay"));
}

function handleMaximizeMouseEnter() {
	if (!isWindows()) return;
	snapOverlayTimer = setTimeout(showSnapOverlay, 620);
}

function handleMaximizeMouseLeave() {
	if (!isWindows()) return;
	if (snapOverlayTimer != null) clearTimeout(snapOverlayTimer);
}

window.onResized(async (_) => {
	isMaximized = await window.isMaximized();
});
</script>

<!-- FIXME: Windows and Linux Title Bar -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed top-0 left-0 w-full h-8 z-[99999] grid grid-cols-[1fr_auto]">
    <div class="h-full w-full" onmousedown={onMouseDown}></div>
    {#if !isMacos()}
        <div class="pe-3">
            <button class="tb-button" 
                onclick={() => window.minimize()}>
                &#59681;
            </button>
            <button class="tb-button" 
                onmouseenter={handleMaximizeMouseEnter}
                onmouseleave={handleMaximizeMouseLeave}
                onclick={() => window.maximize()}>
                &#59682;
            </button>
            <button class="tb-button" 
                onclick={() => window.close()}>
                &#59579;
            </button>
        </div>
    {/if}
</div>

<style lang="scss">
    .tb-button {
        @apply hover:bg-gray-300/25 py-2 px-3 rounded text-[10px] font-[300];
		transition: background 0.1s;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		font-family: 'Segoe Fluent Icons', 'Segoe MDL2 Assets';
    }
</style>