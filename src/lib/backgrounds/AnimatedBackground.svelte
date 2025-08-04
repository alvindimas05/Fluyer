<script lang="ts">
import MusicController from "$lib/controllers/MusicController";
import "./background.scss";
import LoadingController from "$lib/controllers/LoadingController";
import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";
import {onDestroy, onMount} from "svelte";
import * as StackBlur from "stackblur-canvas";
import ColorThief from "colorthief/dist/color-thief.mjs";
import {
	settingAnimatedBackgroundType,
	settingTriggerAnimatedBackground,
} from "$lib/stores/setting";
import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";
import { prominent } from "color.js";
import type {Unsubscriber} from "svelte/store";

const CANVAS_BLOCK_SIZE = 150;
const CANVAS_TRANSITION_DURATION = 750;
const CANVAS_BLUR_RADIUS = 200;

let previousBackground: string | null = null;
let canvas: HTMLCanvasElement;
let canvasContext: CanvasRenderingContext2D;
let currentCanvas: HTMLCanvasElement | null = null;
let newCanvas: HTMLCanvasElement | null = null;
let previousColors: string[] = [];
let previousBackgroundColors: string[][] = [];

function hexToRgb(hex: string): { r: number; g: number; b: number } {
	const bigint = parseInt(hex.slice(1), 16);
	const r = (bigint >> 16) & 255;
	const g = (bigint >> 8) & 255;
	const b = bigint & 255;
	return { r, g, b };
}

function darkenTooBright(hex: string, targetMax = 180): string {
	const { r, g, b } = hexToRgb(hex);
	const luminance = getLuminance(r, g, b);

	if (luminance <= 0.5 && Math.max(r, g, b) <= targetMax) return hex;

	const maxChannel = Math.max(r, g, b);
	const scale = maxChannel > targetMax ? targetMax / maxChannel : 1;

	const newR = Math.floor(r * scale);
	const newG = Math.floor(g * scale);
	const newB = Math.floor(b * scale);

	return rgbToHex(newR, newG, newB);
}

function rgbToHex(r: number, g: number, b: number): string {
	return (
		"#" +
		[r, g, b]
			.map((v) => v.toString(16).padStart(2, "0"))
			.join("")
			.toUpperCase()
	);
}

function getLuminance(r: number, g: number, b: number): number {
	const a = [r, g, b].map((v) => {
		v /= 255;
		return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
	});
	return 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2];
}

async function getColors(force = false): Promise<string[] | null> {
	const currentAlbumImage = await MusicController.currentMusicAlbumImage();
	if (previousBackground === currentAlbumImage && !force) return null;

	let image = new Image();
	image.src = previousBackground = currentAlbumImage;
	if (!image.complete) {
		await new Promise((resolve, reject) => {
			image.onload = () => resolve();
			image.onerror = (err) => reject(err);
		});
	}

	let colors: string[] = [];
	if (
		$settingAnimatedBackgroundType === SettingAnimatedBackgroundType.Prominent
	) {
		// @ts-ignore
		colors = await prominent(image, {
			amount: 10,
			format: "hex",
		});
	} else {
		const colorThief = new ColorThief();
		colors = (await colorThief.getPalette(image, 10)).map((rgb) =>
			rgbToHex(rgb[0], rgb[1], rgb[2]),
		);
	}

	colors = colors.map((hex) => darkenTooBright(hex));
	return colors;
}

async function createCanvas(
	options = {
		usePreviousColors: false,
		force: false,
	},
): Promise<HTMLCanvasElement | null> {
	const canvasBlockSize = CANVAS_BLOCK_SIZE;

	if (!options.usePreviousColors) {
		const colors = await getColors(options.force);
		if (colors === null) return null;

		previousColors = colors;
		previousBackgroundColors = [];
	}

	const rows = Math.ceil(canvas.height / canvasBlockSize);
	const cols = Math.ceil(canvas.width / canvasBlockSize);

	for (let y = 0; y < rows; y++) {
		if (!previousBackgroundColors[y]) {
			previousBackgroundColors[y] = [];
		}

		for (let x = 0; x < cols; x++) {
			if (!previousBackgroundColors[y][x]) {
				previousBackgroundColors[y][x] =
					previousColors[Math.floor(Math.random() * previousColors.length)];
			}
		}
	}

	const baseCanvas = document.createElement("canvas");
	baseCanvas.width = canvas.width;
	baseCanvas.height = canvas.height;
	const baseContext = baseCanvas.getContext("2d")!;

	for (let y = 0; y < rows; y++) {
		for (let x = 0; x < cols; x++) {
			baseContext.fillStyle = previousBackgroundColors[y][x];
			baseContext.fillRect(
				x * canvasBlockSize,
				y * canvasBlockSize,
				canvasBlockSize,
				canvasBlockSize,
			);
		}
	}

	StackBlur.canvasRGBA(
		baseCanvas,
		0,
		0,
		canvas.width,
		canvas.height,
		CANVAS_BLUR_RADIUS,
	);

	return baseCanvas;
}

async function initializeCanvas(reinitialize = false) {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;

	currentCanvas = await createCanvas({ usePreviousColors: reinitialize });

	if (reinitialize) {
		canvasContext.clearRect(0, 0, canvas.width, canvas.height);
		canvasContext.drawImage(currentCanvas!, 0, 0);
	} else {
		canvasContext = canvas.getContext("2d")!;

		const startTime = performance.now();

		function fadeIn(currentTime: number) {
			const elapsed = currentTime - startTime;
			const alpha = Math.min(elapsed / CANVAS_TRANSITION_DURATION, 1);

			canvasContext.clearRect(0, 0, canvas.width, canvas.height);
			canvasContext.globalAlpha = alpha;
			canvasContext.drawImage(currentCanvas!, 0, 0);
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
	LoadingController.setLoadingBackground(true);
}

async function transitionToNewCanvas(force = false) {
	const _newCanvas = await createCanvas({ force });

	if (!_newCanvas || newCanvas !== null) return;

	newCanvas = _newCanvas;

	const width = canvas.width;
	const height = canvas.height;

	const buffer = document.createElement("canvas");
	buffer.width = width;
	buffer.height = height;
	const bufferContext = buffer.getContext("2d")!;

	const startTime = performance.now();

	function animate(currentTime: number) {
		let elapsed = currentTime - startTime;
		if (elapsed < 0) elapsed = 0;
		const progress = Math.min(elapsed / CANVAS_TRANSITION_DURATION, 1);

		bufferContext.clearRect(0, 0, width, height);

		bufferContext.globalAlpha = 1;
		bufferContext.drawImage(currentCanvas!, 0, 0, width, height);

		bufferContext.globalAlpha = progress;
		bufferContext.drawImage(newCanvas!, 0, 0, width, height);

		bufferContext.globalAlpha = 1;
		canvasContext.clearRect(0, 0, width, height);
		canvasContext.drawImage(buffer, 0, 0);

		if (progress < 1) {
			requestAnimationFrame(animate);
		} else {
			currentCanvas = newCanvas;
			newCanvas = null;
		}
	}

	requestAnimationFrame(animate);
}

function onWindowResize() {
	initializeCanvas(true);
}

let isMounted = false;

let unlistenMusicCurrentIndex: Unsubscriber;
let unlistenMusicPlaylist: Unsubscriber;
let unlistenSettingTriggerAnimatedBackground: Unsubscriber;

onMount(() => {
	initializeCanvas();
	unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(() => setTimeout(transitionToNewCanvas, 0));
	unlistenMusicPlaylist = musicPlaylist.subscribe(() => setTimeout(transitionToNewCanvas, 0));
	unlistenSettingTriggerAnimatedBackground = settingTriggerAnimatedBackground.subscribe(() => {
		if (isMounted) setTimeout(() => transitionToNewCanvas(true), 0);
		else isMounted = true;
	});
});

onDestroy(() => {
    unlistenMusicCurrentIndex();
    unlistenMusicPlaylist();
    unlistenSettingTriggerAnimatedBackground();
});
</script>

<svelte:window onresize={onWindowResize} />
<canvas class="fixed rounded-windows" bind:this={canvas}></canvas>