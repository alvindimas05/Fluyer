<script lang="ts">
    import { isDesktop, isLinux, isWindows } from "$lib/platform";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { invoke } from "@tauri-apps/api/core";
    import { PageRoutes } from "$lib/pages";
    import { afterNavigate } from "$app/navigation";

    const LINUX_ICONS = {
        close: "/icons/linux/window-close-symbolic.svg",
        maximize: "/icons/linux/window-maximize-symbolic.svg",
        minimize: "/icons/linux/window-minimize-symbolic.svg",
        restore: "/icons/linux/window-restore-symbolic.svg",
    };

    let isMaximized = $state(true);
    let isPlayPage = $state(false);
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

    afterNavigate((navigation) => {
        isPlayPage = navigation.to?.route.id === PageRoutes.PLAY;
    });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed top-0 left-0 w-full h-8 z-[99999] grid grid-cols-[1fr_auto]">
    <div class="h-full w-full" onmousedown={onMouseDown}></div>
    {#if (isWindows() || isLinux()) && !isPlayPage}
        <div class="pe-3">
            <button
                class={`tb-button ${isWindows() && "win-button"} ${isLinux() && "linux-button"}`}
                onclick={() => window.minimize()}
            >
                {#if isWindows()}
                    &#59681;
                {/if}
                {#if isLinux()}
                    <!-- svelte-ignore a11y_missing_attribute -->
                    <img src={LINUX_ICONS.minimize} />
                {/if}
            </button>
            <button
                class={`tb-button ${isWindows() && "win-button"} ${isLinux() && "linux-button"}`}
                onmouseenter={handleMaximizeMouseEnter}
                onmouseleave={handleMaximizeMouseLeave}
                onclick={() => window.toggleMaximize()}
            >
                {#if isWindows()}
                    {#if isMaximized}
                        &#59683;
                    {:else}
                        &#59682;
                    {/if}
                {/if}
                {#if isLinux()}
                    {#if isMaximized}
                        <!-- svelte-ignore a11y_missing_attribute -->
                        <img src={LINUX_ICONS.restore} />
                    {:else}
                        <!-- svelte-ignore a11y_missing_attribute -->
                        <img src={LINUX_ICONS.maximize} />
                    {/if}
                {/if}
            </button>
            <button
                class={`tb-button ${isWindows() && "win-button"} ${isLinux() && "linux-button"}`}
                onclick={() => window.close()}
            >
                {#if isWindows()}
                    &#59579;
                {/if}
                {#if isLinux()}
                    <!-- svelte-ignore a11y_missing_attribute -->
                    <img src={LINUX_ICONS.close} />
                {/if}
            </button>
        </div>
    {/if}
</div>

<style lang="scss">
    .tb-button {
        @apply hover:bg-gray-300/25 rounded text-[10px] font-[300];
    }
    .win-button {
        @apply py-2 px-3;
        transition: background 0.1s;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        font-family: "Segoe Fluent Icons", "Segoe MDL2 Assets";
    }
    .linux-button {
        @apply p-1;
        img {
            width: 1.25rem;
        }
    }
</style>
