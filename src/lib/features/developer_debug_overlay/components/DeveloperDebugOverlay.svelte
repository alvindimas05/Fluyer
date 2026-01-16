<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

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

	function toggleVisibility() {
		visible = !visible;
	}

	// Drag functionality
	function handleMouseDown(e: MouseEvent) {
		if (e.target !== e.currentTarget) return; // Only drag from header
		isDragging = true;
		dragStartX = e.clientX - overlayX;
		dragStartY = e.clientY - overlayY;

		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
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
		document.removeEventListener('mousemove', handleMouseMove);
		document.removeEventListener('mouseup', handleMouseUp);
	}

	function handleTouchStart(e: TouchEvent) {
		if (e.target !== e.currentTarget) return; // Only drag from header
		const touch = e.touches[0];
		isDragging = true;
		dragStartX = touch.clientX - overlayX;
		dragStartY = touch.clientY - overlayY;

		document.addEventListener('touchmove', handleTouchMove, { passive: false });
		document.addEventListener('touchend', handleTouchEnd);
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
		document.removeEventListener('touchmove', handleTouchMove);
		document.removeEventListener('touchend', handleTouchEnd);
	}

	onMount(() => {
		// Initialize overlay position (top-right by default)
		overlayX = window.innerWidth - 340; // 320px width + 20px margin
		overlayY = 80; // 5rem from top

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

		// Clean up event listeners
		document.removeEventListener('mousemove', handleMouseMove);
		document.removeEventListener('mouseup', handleMouseUp);
		document.removeEventListener('touchmove', handleTouchMove);
		document.removeEventListener('touchend', handleTouchEnd);
	});
</script>

{#if visible}
	<div
		bind:this={overlayElement}
		class="fixed z-[9999] w-80 rounded-md border border-gray-700 bg-black/90 font-mono text-xs text-green-400 shadow-2xl backdrop-blur-sm {isDragging
			? 'cursor-grabbing'
			: ''}"
		style="left: {overlayX}px; top: {overlayY}px; transform: translate(0, 0);"
	>
		<div
			class="flex cursor-grab select-none items-center justify-between border-b border-gray-700 bg-green-400/10 px-3 py-2 font-bold {isDragging
				? 'cursor-grabbing'
				: 'cursor-grab'}"
			onmousedown={handleMouseDown}
			ontouchstart={handleTouchStart}
		>
			<span>🔧 Debug Overlay</span>
			<button
				class="flex h-5 w-5 cursor-pointer items-center justify-center text-base text-green-400 transition-colors hover:rounded hover:bg-red-500/20"
				onclick={toggleVisibility}>×</button
			>
		</div>

		<div
			class="scrollbar-thin scrollbar-thumb-green-400 scrollbar-track-white/10 max-h-96 overflow-y-auto p-3"
		>
			<div class="mb-4">
				<h4 class="mb-2 border-b border-gray-700 pb-1 text-sm text-cyan-400">Performance</h4>
				<div class="flex items-center justify-between py-0.5">
					<span class="flex-shrink-0 text-gray-300">FPS:</span>
					<span
						class="break-all text-right font-bold {fps < 30
							? 'text-red-400'
							: fps >= 60
								? 'text-green-400'
								: 'text-green-400'}">{fps}</span
					>
				</div>
				{#if 'memory' in performance}
					<div class="flex items-center justify-between py-0.5">
						<span class="flex-shrink-0 text-gray-300">RAM Usage:</span>
						<span class="text-right font-bold text-green-400">{ramUsage} MB / {ramLimit} MB</span>
					</div>
					<div class="flex items-center justify-between py-0.5">
						<span class="flex-shrink-0 text-gray-300">RAM %:</span>
						<span
							class="text-right font-bold {ramUsage / ramLimit > 0.8
								? 'text-yellow-400'
								: 'text-green-400'}">{Math.round((ramUsage / ramLimit) * 100)}%</span
						>
					</div>
				{/if}

				{#if loadTime > 0}
					<div class="flex items-center justify-between py-0.5">
						<span class="flex-shrink-0 text-gray-300">Page Load:</span>
						<span class="text-right font-bold text-green-400">{loadTime} ms</span>
					</div>
				{/if}
			</div>

			<div class="mb-4">
				<h4 class="mb-2 border-b border-gray-700 pb-1 text-sm text-cyan-400">System Info</h4>
				<div class="flex items-center justify-between py-0.5">
					<span class="flex-shrink-0 text-gray-300">User Agent:</span>
					<span
						class="max-w-[150px] overflow-hidden text-ellipsis whitespace-nowrap text-right text-[10px] font-bold text-green-400"
					>
						{navigator.userAgent.split(' ')[0]}
					</span>
				</div>
				<div class="flex items-center justify-between py-0.5">
					<span class="flex-shrink-0 text-gray-300">Connection:</span>
					<span class="text-right font-bold text-green-400">{getConnectionType()}</span>
				</div>
				<div class="flex items-center justify-between py-0.5">
					<span class="flex-shrink-0 text-gray-300">Viewport:</span>
					<span class="text-right font-bold text-green-400"
						>{window.innerWidth}×{window.innerHeight}</span
					>
				</div>
			</div>

			<div class="mb-0">
				<h4 class="mb-2 border-b border-gray-700 pb-1 text-sm text-cyan-400">Timing</h4>
				<div class="flex items-center justify-between py-0.5">
					<span class="flex-shrink-0 text-gray-300">Timestamp:</span>
					<span class="text-right text-[10px] font-bold text-green-400"
						>{new Date(timestamp).toLocaleTimeString()}</span
					>
				</div>
				{#if navigationEntry}
					<div class="flex items-center justify-between py-0.5">
						<span class="flex-shrink-0 text-gray-300">DOM Ready:</span>
						<span class="text-right font-bold text-green-400"
							>{Math.round(navigationEntry.duration - navigationEntry.startTime)} ms</span
						>
					</div>
				{/if}
			</div>
		</div>
	</div>
{:else}
	<button
		class="fixed right-3 top-[3.05rem] z-[9999] flex h-10 w-10 cursor-pointer items-center justify-center rounded-full border border-gray-700 bg-black/80 text-base text-green-400 backdrop-blur-sm transition-all hover:bg-gray-700"
		onclick={toggleVisibility}
		title="Show Debug Overlay (Ctrl+Shift+D)"
	>
		🔧
	</button>
{/if}
