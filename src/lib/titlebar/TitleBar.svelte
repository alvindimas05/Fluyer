<script lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import TitleBarButton from "./TitleBarButton.svelte";
import { platform } from "@tauri-apps/plugin-os";
import { isMobile } from "$lib/platform";

let isMaximized = $state(true);
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
appWindow.onResized(async (_) => {
	isMaximized = await appWindow.isMaximized();
});
</script>

{#if !isMobile()}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="fixed top-0 left-0 w-full h-8 z-[99999] grid grid-cols-[1fr_auto]"
    >
        <div class="h-full w-full" onmousedown={onMouseDown}></div>
        {#if platform() != "macos"}
            <div class="pe-3">
                <TitleBarButton
                    name="minimize"
                    onclick={() => appWindow.minimize()}
                />
                <TitleBarButton
                    name={isMaximized ? "shrink" : "expand"}
                    onclick={() => appWindow.toggleMaximize()}
                />
                <TitleBarButton
                    name="close"
                    onclick={() => appWindow.close()}
                />
            </div>
        {/if}
    </div>
{/if}
