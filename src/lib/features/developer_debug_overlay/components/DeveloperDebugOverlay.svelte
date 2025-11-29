<script lang="ts">
import { onDestroy, onMount } from "svelte";

let visible = $state(false);
let fps = $state(0);
let ramUsage = $state(0);
let ramLimit = $state(0);
let loadTime = $state(0);
let timestamp = $state(Date.now());
let frameCount = $state(0);
let lastTime = $state(performance.now());

let animationFrame = $state(0);
let fpsInterval = $state<ReturnType<typeof setInterval> | null>(null);

// Drag functionality state
let isDragging = $state(false);
let dragStartX = $state(0);
let dragStartY = $state(0);
let overlayX = $state(0);
let overlayY = $state(0);
let overlayElement = $state<HTMLElement | null>(null);

// Performance observer for navigation timing
let navigationEntry = $state<PerformanceEntry | null>(null);

function startFPSMonitoring() {
    function calculateFPS(currentTime: number) {
        frameCount++;

        if (currentTime >= lastTime + 1000) {
            fps = Math.round((frameCount * 1000) / (currentTime - lastTime));
            frameCount = 0;
            lastTime = currentTime;
        }

        animationFrame = requestAnimationFrame(calculateFPS);
    }

    animationFrame = requestAnimationFrame(calculateFPS);
}

function updateMetrics() {
    timestamp = Date.now();

    // Get memory usage if available
    if ("memory" in performance) {
        ramUsage = Math.round(performance.memory.usedJSHeapSize / 1024 / 1024);
        ramLimit = Math.round(performance.memory.jsHeapSizeLimit / 1024 / 1024);
    }
}

function getConnectionType() {
    if ("connection" in navigator) {
        return navigator.connection.effectiveType || "unknown";
    }
    return "unknown";
}

function toggleVisibility() {
    visible = !visible;
}

// Drag functionality
function handleMouseDown(e: MouseEvent) {
    if (e.target !== e.currentTarget) return; // Only drag from header
    isDragging = true;
    dragStartX = e.clientX - overlayX;
    dragStartY = e.clientY - overlayY;

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
    e.preventDefault();
}

function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;

    const newX = e.clientX - dragStartX;
    const newY = e.clientY - dragStartY;

    // Constrain to viewport
    const maxX = window.innerWidth - (overlayElement?.offsetWidth || 320);
    const maxY = window.innerHeight - (overlayElement?.offsetHeight || 200);

    overlayX = Math.max(0, Math.min(newX, maxX));
    overlayY = Math.max(0, Math.min(newY, maxY));
}

function handleMouseUp() {
    isDragging = false;
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
}

function handleTouchStart(e: TouchEvent) {
    if (e.target !== e.currentTarget) return; // Only drag from header
    const touch = e.touches[0];
    isDragging = true;
    dragStartX = touch.clientX - overlayX;
    dragStartY = touch.clientY - overlayY;

    document.addEventListener("touchmove", handleTouchMove, { passive: false });
    document.addEventListener("touchend", handleTouchEnd);
    e.preventDefault();
}

function handleTouchMove(e: TouchEvent) {
    if (!isDragging) return;

    const touch = e.touches[0];
    const newX = touch.clientX - dragStartX;
    const newY = touch.clientY - dragStartY;

    // Constrain to viewport
    const maxX = window.innerWidth - (overlayElement?.offsetWidth || 320);
    const maxY = window.innerHeight - (overlayElement?.offsetHeight || 200);

    overlayX = Math.max(0, Math.min(newX, maxX));
    overlayY = Math.max(0, Math.min(newY, maxY));

    e.preventDefault();
}

function handleTouchEnd() {
    isDragging = false;
    document.removeEventListener("touchmove", handleTouchMove);
    document.removeEventListener("touchend", handleTouchEnd);
}

onMount(() => {
    // Initialize overlay position (top-right by default)
    overlayX = window.innerWidth - 340; // 320px width + 20px margin
    overlayY = 80; // 5rem from top

    // Get page load time
    if (performance.getEntriesByType) {
        const navEntries = performance.getEntriesByType("navigation");
        if (navEntries.length > 0) {
            navigationEntry = navEntries[0];
            // loadTime = Math.round(navigationEntry.loadEventEnd - navigationEntry.loadEventStart);
        }
    }

    // Start FPS monitoring
    startFPSMonitoring();

    // Update RAM usage periodically
    fpsInterval = setInterval(updateMetrics, 1000);
});

onDestroy(() => {
    if (animationFrame) {
        cancelAnimationFrame(animationFrame);
    }
    if (fpsInterval) {
        clearInterval(fpsInterval);
    }

    // Clean up event listeners
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
    document.removeEventListener("touchmove", handleTouchMove);
    document.removeEventListener("touchend", handleTouchEnd);
});
</script>

{#if visible}
    <div
            bind:this={overlayElement}
            class="fixed w-80 bg-black/90 text-green-400 font-mono text-xs border border-gray-700 rounded-md z-[9999] backdrop-blur-sm shadow-2xl {isDragging ? 'cursor-grabbing' : ''}"
            style="left: {overlayX}px; top: {overlayY}px; transform: translate(0, 0);"
    >
        <div
                class="flex justify-between items-center px-3 py-2 bg-green-400/10 border-b border-gray-700 font-bold cursor-grab select-none {isDragging ? 'cursor-grabbing' : 'cursor-grab'}"
                onmousedown={handleMouseDown}
                ontouchstart={handleTouchStart}
        >
            <span>🔧 Debug Overlay</span>
            <button
                    class="w-5 h-5 flex items-center justify-center text-green-400 text-base hover:bg-red-500/20 hover:rounded transition-colors cursor-pointer"
                    onclick={toggleVisibility}
            >×</button>
        </div>

        <div class="p-3 max-h-96 overflow-y-auto scrollbar-thin scrollbar-thumb-green-400 scrollbar-track-white/10">
            <div class="mb-4">
                <h4 class="text-cyan-400 mb-2 text-sm border-b border-gray-700 pb-1">Performance</h4>
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">FPS:</span>
                    <span class="font-bold text-right break-all {fps < 30 ? 'text-red-400' : fps >= 60 ? 'text-green-400' : 'text-green-400'}">{fps}</span>
                </div>
                {#if 'memory' in performance}
                    <div class="flex justify-between items-center py-0.5">
                        <span class="text-gray-300 flex-shrink-0">RAM Usage:</span>
                        <span class="font-bold text-right text-green-400">{ramUsage} MB / {ramLimit} MB</span>
                    </div>
                    <div class="flex justify-between items-center py-0.5">
                        <span class="text-gray-300 flex-shrink-0">RAM %:</span>
                        <span class="font-bold text-right {ramUsage / ramLimit > 0.8 ? 'text-yellow-400' : 'text-green-400'}">{Math.round((ramUsage / ramLimit) * 100)}%</span>
                    </div>
                {/if}

                {#if loadTime > 0}
                    <div class="flex justify-between items-center py-0.5">
                        <span class="text-gray-300 flex-shrink-0">Page Load:</span>
                        <span class="font-bold text-right text-green-400">{loadTime} ms</span>
                    </div>
                {/if}
            </div>

            <div class="mb-4">
                <h4 class="text-cyan-400 mb-2 text-sm border-b border-gray-700 pb-1">System Info</h4>
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">User Agent:</span>
                    <span class="font-bold text-right text-green-400 text-[10px] max-w-[150px] overflow-hidden text-ellipsis whitespace-nowrap">
                        {navigator.userAgent.split(' ')[0]}
                    </span>
                </div>
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">Connection:</span>
                    <span class="font-bold text-right text-green-400">{getConnectionType()}</span>
                </div>
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">Viewport:</span>
                    <span class="font-bold text-right text-green-400">{window.innerWidth}×{window.innerHeight}</span>
                </div>
            </div>

            <div class="mb-0">
                <h4 class="text-cyan-400 mb-2 text-sm border-b border-gray-700 pb-1">Timing</h4>
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">Timestamp:</span>
                    <span class="font-bold text-right text-green-400 text-[10px]">{new Date(timestamp).toLocaleTimeString()}</span>
                </div>
                {#if navigationEntry}
                    <div class="flex justify-between items-center py-0.5">
                        <span class="text-gray-300 flex-shrink-0">DOM Ready:</span>
                        <span class="font-bold text-right text-green-400">{Math.round(navigationEntry.duration - navigationEntry.startTime)} ms</span>
                    </div>
                {/if}
            </div>
        </div>
    </div>
{:else}
    <button
            class="fixed top-[3.05rem] right-3 w-10 h-10 bg-black/80 border border-gray-700 rounded-full text-green-400 text-base cursor-pointer z-[9999] backdrop-blur-sm flex items-center justify-center transition-all hover:bg-gray-700"
            onclick={toggleVisibility}
            title="Show Debug Overlay (Ctrl+Shift+D)"
    >
        🔧
    </button>
{/if}