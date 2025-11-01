<script lang="ts">
import MusicController from "$lib/controllers/MusicController";
import "./background.scss";
import LoadingController from "$lib/controllers/LoadingController";
import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";
import { onDestroy, onMount } from "svelte";
import * as StackBlur from "stackblur-canvas";
import ColorThief from "colorthief/dist/color-thief.mjs";
import {
	settingAnimatedBackgroundType,
	settingTriggerAnimatedBackground,
} from "$lib/stores/setting";
import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";
import { prominent } from "color.js";
import type { Unsubscriber } from "svelte/store";
import {page} from "$app/state";
import {PageRoutes} from "$lib/pages";
import {MusicSize} from "$lib/home/music/types";

const SCALE = 0.05;
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

function balanceColor(hex: string): string {
    const { r, g, b } = hexToRgb(hex);
    const hsl = rgbToHsl(r, g, b);

    // Balance saturation - reduce overly vibrant colors
    if (hsl.s > 0.7) {
        // Very saturated colors - tone them down significantly
        hsl.s = 0.5 + (hsl.s - 0.7) * 0.4;
    }

    // Balance lightness for better visibility
    if (hsl.l > 0.7) {
        // Too bright - darken more aggressively
        hsl.l = 0.45 + (hsl.l - 0.7) * 0.3;
    }

    // Ensure we stay within reasonable bounds
    hsl.s = Math.max(0.15, Math.min(0.65, hsl.s));
    hsl.l = Math.max(0.2, Math.min(0.6, hsl.l));

    const rgb = hslToRgb(hsl.h, hsl.s, hsl.l);
    return rgbToHex(rgb.r, rgb.g, rgb.b);
}

function rgbToHsl(r: number, g: number, b: number): { h: number, s: number, l: number } {
    r /= 255;
    g /= 255;
    b /= 255;

    const max = Math.max(r, g, b);
    const min = Math.min(r, g, b);
    let h = 0, s = 0, l = (max + min) / 2;

    if (max !== min) {
        const d = max - min;
        s = l > 0.5 ? d / (2 - max - min) : d / (max + min);

        switch (max) {
            case r: h = (g - b) / d + (g < b ? 6 : 0); break;
            case g: h = (b - r) / d + 2; break;
            case b: h = (r - g) / d + 4; break;
        }
        h /= 6;
    }

    return { h, s, l };
}

function hslToRgb(h: number, s: number, l: number): { r: number, g: number, b: number } {
    let r, g, b;

    if (s === 0) {
        r = g = b = l;
    } else {
        const hue2rgb = (p: number, q: number, t: number) => {
            if (t < 0) t += 1;
            if (t > 1) t -= 1;
            if (t < 1/6) return p + (q - p) * 6 * t;
            if (t < 1/2) return q;
            if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
            return p;
        };

        const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
        const p = 2 * l - q;

        r = hue2rgb(p, q, h + 1/3);
        g = hue2rgb(p, q, h);
        b = hue2rgb(p, q, h - 1/3);
    }

    return {
        r: Math.round(r * 255),
        g: Math.round(g * 255),
        b: Math.round(b * 255)
    };
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

async function getColors(force = false): Promise<string[] | null> {
    const currentAlbumImage = await MusicController.getAlbumImageFromMusic(
        MusicController.currentMusic,
        MusicSize.AnimatedBackground,
    );
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

    colors = colors.map((hex) => balanceColor(hex));
    return colors;
}

async function createCanvas(
    options = {
        usePreviousColors: false,
        force: false,
    },
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

    const baseCanvas = document.createElement("canvas");
    baseCanvas.width = scaledWidth;
    baseCanvas.height = scaledHeight;
    const baseContext = baseCanvas.getContext("2d")!;

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

	currentCanvas = await createCanvas({ usePreviousColors: reinitialize });

	if (reinitialize) {
		canvasContext.clearRect(0, 0, canvas.width, canvas.height);
		canvasContext.drawImage(currentCanvas!, 0, 0, canvas.width, canvas.height);
	} else {
		canvasContext = canvas.getContext("2d")!;

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
		canvasContext.drawImage(buffer, 0, 0, width, height);

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
	unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(() =>
		setTimeout(transitionToNewCanvas, 0),
	);
	unlistenMusicPlaylist = musicPlaylist.subscribe(() =>
		setTimeout(transitionToNewCanvas, 0),
	);
	unlistenSettingTriggerAnimatedBackground =
		settingTriggerAnimatedBackground.subscribe(() => {
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
<canvas class="fixed rounded-windows"
        class:hidden={page.url.pathname === PageRoutes.VISUALIZER}
        bind:this={canvas}></canvas>