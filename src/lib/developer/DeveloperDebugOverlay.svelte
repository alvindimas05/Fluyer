<script lang="ts">
import {onDestroy, onMount} from "svelte";

let fps = $state(0);
let ramUsage = $state(0);
let ramLimit = $state(0);
let loadTime = $state(0);
let timestamp = $state(Date.now());
let frameCount = $state(0);
let lastTime = $state(performance.now());

let animationFrame = $state(0);
let fpsInterval = $state<ReturnType<typeof setInterval> | null>(null);

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
    if ('memory' in performance) {
        ramUsage = Math.round(performance.memory.usedJSHeapSize / 1024 / 1024);
        ramLimit = Math.round(performance.memory.jsHeapSizeLimit / 1024 / 1024);
    }
}

function getConnectionType() {
    if ('connection' in navigator) {
        return navigator.connection.effectiveType || 'unknown';
    }
    return 'unknown';
}

onMount(() => {
    // Get page load time
    if (performance.getEntriesByType) {
        const navEntries = performance.getEntriesByType('navigation');
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
});
</script>

<div class="fixed top-5 right-5 w-80 bg-black/90 text-green-400 font-mono text-xs border border-gray-700 rounded-md z-[9999] backdrop-blur-sm shadow-2xl">
    <div class="flex justify-between items-center px-3 py-2 bg-green-400/10 border-b border-gray-700 font-bold">
        <span>🔧 Debug Overlay</span>
    </div>

    <div class="p-3 max-h-96 overflow-y-auto scrollbar-thin scrollbar-thumb-green-400 scrollbar-track-white/10">
        <div class="mb-4">
            <h4 class="text-cyan-400 mb-2 text-sm border-b border-gray-700 pb-1">Performance</h4>
            <div class="flex justify-between items-center py-0.5">
                <span class="text-gray-300 flex-shrink-0">FPS:</span>
                <span class="font-bold text-right break-all {fps < 30 ? 'text-red-400' : fps >= 60 ? 'text-green-400' : 'text-green-400'}">
        {fps}
      </span>
            </div>

            {#if 'memory' in performance}
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">RAM Usage:</span>
                    <span class="font-bold text-right text-green-400">{ramUsage} MB / {ramLimit} MB</span>
                </div>
                <div class="flex justify-between items-center py-0.5">
                    <span class="text-gray-300 flex-shrink-0">RAM %:</span>
                    <span class="font-bold text-right {ramUsage / ramLimit > 0.8 ? 'text-yellow-400' : 'text-green-400'}">
          {Math.round((ramUsage / ramLimit) * 100)}%
        </span>
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