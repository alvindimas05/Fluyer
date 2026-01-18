<script lang="ts">
	import '../style.scss';
	import { onDestroy, onMount } from 'svelte';
	import * as StackBlur from 'stackblur-canvas';
	// @ts-ignore
	import ColorThief from 'colorthief/dist/color-thief.mjs';
	import { SettingAnimatedBackgroundType } from '$lib/features/settings/animated_background/types';
	import { prominent } from 'color.js';
	import { page } from '$app/state';
	import { PageRoutes } from '$lib/constants/PageRoutes';
	import { isLinux, isMobile } from '$lib/platform';
	import { afterNavigate } from '$app/navigation';
	import { MusicConfig } from '$lib/constants/MusicConfig';
	import MetadataService from '$lib/services/MetadataService.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import settingStore from '$lib/stores/setting.svelte';
	import LibraryService from '$lib/services/LibraryService.svelte';

	const SCALE = isMobile() ? 0.03 : 0.05;
	const CANVAS_BLOCK_SIZE = $derived(window.innerWidth > 640 ? 150 : 100);
	const CANVAS_TRANSITION_DURATION = 750;
	const CANVAS_BLUR_RADIUS = isMobile() ? 100 : 200;

	let previousBackground: string | null = null;
	let canvas: HTMLCanvasElement;
	let canvasContext: CanvasRenderingContext2D;
	let currentCanvas: HTMLCanvasElement | null = null;
	let previousColors: string[] = [];
	let previousBackgroundColors: string[][] = [];
	let currentAlbumImage: string | null = null;

	// Transition management
	let targetCanvas: HTMLCanvasElement | null = null;
	let activeTransition: number | null = null;
	let transitionProgress: number = 0;

	// Performance optimization: reuse canvases
	const canvasPool: HTMLCanvasElement[] = [];
	const MAX_POOL_SIZE = 3;

	function getCanvasFromPool(width: number, height: number): HTMLCanvasElement {
		let canvas = canvasPool.pop();
		if (!canvas) {
			canvas = document.createElement('canvas');
		}
		canvas.width = width;
		canvas.height = height;
		return canvas;
	}

	function returnCanvasToPool(canvas: HTMLCanvasElement) {
		if (canvasPool.length < MAX_POOL_SIZE) {
			canvasPool.push(canvas);
		}
	}

	function hexToRgb(hex: string): { r: number; g: number; b: number } {
		const bigint = parseInt(hex.slice(1), 16);
		const r = (bigint >> 16) & 255;
		const g = (bigint >> 8) & 255;
		const b = bigint & 255;
		return { r, g, b };
	}

	function balanceColor(hex: string): string {
		const { r, g, b } = hexToRgb(hex);
		const hsl = rgbToHsl(r, g, b);

		// Balance lightness for better visibility
		if (hsl.l > 0.7) {
			hsl.l = 0.45 + (hsl.l - 0.7) * 0.3;
		}

		// Ensure we stay within reasonable bounds
		if (currentAlbumImage === MusicConfig.defaultAlbumImage) {
			hsl.s = 0.6;
			hsl.l = 0.6;
		} else {
			hsl.l = Math.max(0.1, Math.min(0.7, hsl.l));
		}

		const rgb = hslToRgb(hsl.h, hsl.s, hsl.l);
		return rgbToHex(rgb.r, rgb.g, rgb.b);
	}

	function rgbToHsl(r: number, g: number, b: number): { h: number; s: number; l: number } {
		r /= 255;
		g /= 255;
		b /= 255;

		const max = Math.max(r, g, b);
		const min = Math.min(r, g, b);
		let h = 0,
			s = 0,
			l = (max + min) / 2;

		if (max !== min) {
			const d = max - min;
			s = l > 0.5 ? d / (2 - max - min) : d / (max + min);

			switch (max) {
				case r:
					h = (g - b) / d + (g < b ? 6 : 0);
					break;
				case g:
					h = (b - r) / d + 2;
					break;
				case b:
					h = (r - g) / d + 4;
					break;
			}
			h /= 6;
		}

		return { h, s, l };
	}

	function hslToRgb(h: number, s: number, l: number): { r: number; g: number; b: number } {
		let r, g, b;

		if (s === 0) {
			r = g = b = l;
		} else {
			const hue2rgb = (p: number, q: number, t: number) => {
				if (t < 0) t += 1;
				if (t > 1) t -= 1;
				if (t < 1 / 6) return p + (q - p) * 6 * t;
				if (t < 1 / 2) return q;
				if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
				return p;
			};

			const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
			const p = 2 * l - q;

			r = hue2rgb(p, q, h + 1 / 3);
			g = hue2rgb(p, q, h);
			b = hue2rgb(p, q, h - 1 / 3);
		}

		return {
			r: Math.round(r * 255),
			g: Math.round(g * 255),
			b: Math.round(b * 255)
		};
	}

	function rgbToHex(r: number, g: number, b: number): string {
		return (
			'#' +
			[r, g, b]
				.map((v) => v.toString(16).padStart(2, '0'))
				.join('')
				.toUpperCase()
		);
	}

	async function getColors(force = false): Promise<string[] | null> {
		currentAlbumImage = await MetadataService.getMusicCoverArt(musicStore.currentMusic);
		if (previousBackground === currentAlbumImage && !force) return null;

		let image = new Image();
		image.crossOrigin = 'anonymous';
		image.src = previousBackground = currentAlbumImage;

		if (!image.complete) {
			await new Promise((resolve, reject) => {
				image.onload = () => resolve(null);
				image.onerror = (err) => reject(err);
			});
		}

		let colors: string[] = [];
		if (settingStore.animatedBackground.type === SettingAnimatedBackgroundType.Prominent) {
			// @ts-ignore
			colors = await prominent(image, {
				amount: 10,
				format: 'hex'
			});
		} else {
			const colorThief = new ColorThief();
			colors = (await colorThief.getPalette(image, 10)).map((rgb: any) =>
				rgbToHex(rgb[0], rgb[1], rgb[2])
			);
		}

		colors = colors.map((hex) => balanceColor(hex));
		return colors;
	}

	async function createCanvas(
		options = {
			usePreviousColors: false,
			force: false
		}
	): Promise<HTMLCanvasElement | null> {
		if (!options.usePreviousColors) {
			const colors = await getColors(options.force);
			if (colors === null) return null;

			previousColors = colors;
			previousBackgroundColors = [];
		}

		const scaledWidth = Math.floor(canvas.width * SCALE);
		const scaledHeight = Math.floor(canvas.height * SCALE);
		const blockSize = Math.floor(CANVAS_BLOCK_SIZE * SCALE);

		const rows = Math.ceil(scaledHeight / blockSize);
		const cols = Math.ceil(scaledWidth / blockSize);

		for (let y = 0; y < rows; y++) {
			if (!previousBackgroundColors[y]) previousBackgroundColors[y] = [];
			for (let x = 0; x < cols; x++) {
				if (!previousBackgroundColors[y][x]) {
					previousBackgroundColors[y][x] =
						previousColors[Math.floor(Math.random() * previousColors.length)];
				}
			}
		}

		const baseCanvas = getCanvasFromPool(scaledWidth, scaledHeight);
		const baseContext = baseCanvas.getContext('2d', { alpha: false })!;

		for (let y = 0; y < rows; y++) {
			for (let x = 0; x < cols; x++) {
				baseContext.fillStyle = previousBackgroundColors[y][x];
				baseContext.fillRect(x * blockSize, y * blockSize, blockSize, blockSize);
			}
		}

		const blurRadius = Math.floor(CANVAS_BLUR_RADIUS * SCALE);
		StackBlur.canvasRGBA(baseCanvas, 0, 0, scaledWidth, scaledHeight, blurRadius);

		return baseCanvas;
	}

	async function initializeCanvas(reinitialize = false) {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;

		const newCanvas = await createCanvas({
			usePreviousColors: reinitialize
		});
		if (!newCanvas) return;

		if (reinitialize) {
			canvasContext.clearRect(0, 0, canvas.width, canvas.height);
			canvasContext.drawImage(newCanvas, 0, 0, canvas.width, canvas.height);

			// Clean up old canvas
			if (currentCanvas) {
				returnCanvasToPool(currentCanvas);
			}
			currentCanvas = newCanvas;
		} else {
			canvasContext = canvas.getContext('2d')!;
			currentCanvas = newCanvas;

			const startTime = performance.now();

			function fadeIn(currentTime: number) {
				let elapsed = currentTime - startTime;
				if (elapsed < 0) elapsed = 0;
				const alpha = Math.min(elapsed / CANVAS_TRANSITION_DURATION, 1);

				canvasContext.clearRect(0, 0, canvas.width, canvas.height);
				canvasContext.globalAlpha = alpha;
				canvasContext.drawImage(currentCanvas!, 0, 0, canvas.width, canvas.height);
				canvasContext.globalAlpha = 1.0;

				if (alpha < 1) {
					requestAnimationFrame(fadeIn);
				} else {
					afterInitializeCanvas();
				}
			}

			requestAnimationFrame(fadeIn);
		}
	}

	async function afterInitializeCanvas() {
		console.log('AnimatedBackground is initialized');
		LibraryService.initialize();
	}

	// Smooth transition function with proper interruption support
	async function transitionToNewCanvas(force = false) {
		const now = performance.now();

		const _newCanvas = await createCanvas({ force });
		if (!_newCanvas) return;

		const width = canvas.width;
		const height = canvas.height;

		// Handle interruption: capture current blended state
		if (activeTransition !== null) {
			cancelAnimationFrame(activeTransition);
			activeTransition = null;

			// Capture the current blended frame as the new starting point
			const capturedCanvas = getCanvasFromPool(width, height);
			const capturedContext = capturedCanvas.getContext('2d', {
				alpha: false
			})!;

			// Draw the current state (blend of currentCanvas and targetCanvas at current progress)
			capturedContext.clearRect(0, 0, width, height);

			if (currentCanvas) {
				capturedContext.globalAlpha = 1;
				capturedContext.drawImage(currentCanvas, 0, 0, width, height);
			}

			if (targetCanvas && transitionProgress > 0) {
				capturedContext.globalAlpha = transitionProgress;
				capturedContext.drawImage(targetCanvas, 0, 0, width, height);
			}

			capturedContext.globalAlpha = 1;

			// Clean up old canvases
			if (currentCanvas) {
				returnCanvasToPool(currentCanvas);
			}
			if (targetCanvas) {
				returnCanvasToPool(targetCanvas);
			}

			// Use captured state as new starting point
			currentCanvas = capturedCanvas;
			targetCanvas = null;
			transitionProgress = 0;
		}

		// Set new target canvas
		if (targetCanvas) {
			returnCanvasToPool(targetCanvas);
		}
		targetCanvas = _newCanvas;

		// Reuse buffer canvas for rendering
		const buffer = getCanvasFromPool(width, height);
		const bufferContext = buffer.getContext('2d', { alpha: false })!;

		const startTime = performance.now();
		let lastProgress = 0;

		function animate(currentTime: number) {
			let elapsed = currentTime - startTime;
			if (elapsed < 0) elapsed = 0;

			// Linear progress
			const progress = Math.min(elapsed / CANVAS_TRANSITION_DURATION, 1);
			transitionProgress = progress;

			// Only update if progress changed significantly (optimization)
			if (Math.abs(progress - lastProgress) < 0.01 && progress < 1) {
				activeTransition = requestAnimationFrame(animate);
				return;
			}
			lastProgress = progress;

			bufferContext.clearRect(0, 0, width, height);

			// Draw current state
			bufferContext.globalAlpha = 1;
			if (currentCanvas) {
				bufferContext.drawImage(currentCanvas, 0, 0, width, height);
			}

			// Blend in target state
			bufferContext.globalAlpha = progress;
			bufferContext.drawImage(targetCanvas!, 0, 0, width, height);

			// Draw to main canvas
			bufferContext.globalAlpha = 1;
			canvasContext.clearRect(0, 0, width, height);
			canvasContext.drawImage(buffer, 0, 0, width, height);

			if (progress < 1) {
				activeTransition = requestAnimationFrame(animate);
			} else {
				// Transition complete
				if (currentCanvas) {
					returnCanvasToPool(currentCanvas);
				}
				currentCanvas = targetCanvas;
				targetCanvas = null;
				transitionProgress = 0;
				activeTransition = null;
				returnCanvasToPool(buffer);

				console.log(`Animated Background transition took ${performance.now() - now} ms`);
			}
		}

		activeTransition = requestAnimationFrame(animate);
	}

	function onWindowResize() {
		initializeCanvas(true);
	}

	let isMounted = false;

	onMount(() => {
		initializeCanvas();

		if (isLinux())
			afterNavigate((navigation) => {
				if (navigation.from?.route.id !== PageRoutes.VISUALIZER) return;
				initializeCanvas(true);
			});

		$effect(() => {
			musicStore.currentIndex;
			musicStore.list;
			transitionToNewCanvas();
		});

		$effect(() => {
			settingStore.animatedBackground.trigger;
			if (isMounted) transitionToNewCanvas(true);
			else isMounted = true;
		});
	});

	onDestroy(() => {
		// Cancel any active transitions
		if (activeTransition !== null) {
			cancelAnimationFrame(activeTransition);
		}

		// Clean up canvases
		if (currentCanvas) returnCanvasToPool(currentCanvas);
		if (targetCanvas) returnCanvasToPool(targetCanvas);

		// Clear pool
		canvasPool.length = 0;
	});
</script>

<svelte:window onresize={onWindowResize} />
<canvas
	class="rounded-windows fixed"
	class:hidden={page.url.pathname === PageRoutes.VISUALIZER}
	bind:this={canvas}
></canvas>
