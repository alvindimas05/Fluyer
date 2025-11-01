<script lang="ts">
import MusicController from "$lib/controllers/MusicController";
import "./background.scss";
import LoadingController from "$lib/controllers/LoadingController";
import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";
import { onDestroy, onMount } from "svelte";
import * as StackBlur from "stackblur-canvas";
// @ts-ignore
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
let previousColors: string[] = [];
let previousBackgroundColors: string[][] = [];

// Transition queue management
let pendingTransition: HTMLCanvasElement | null = null;
let activeTransition: number | null = null;

// Performance optimization: reuse canvases
const canvasPool: HTMLCanvasElement[] = [];
const MAX_POOL_SIZE = 3;

function getCanvasFromPool(width: number, height: number): HTMLCanvasElement {
    let canvas = canvasPool.pop();
    if (!canvas) {
        canvas = document.createElement("canvas");
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

    // Balance saturation - reduce overly vibrant colors
    if (hsl.s > 0.7) {
        hsl.s = 0.5 + (hsl.s - 0.7) * 0.4;
    }

    // Balance lightness for better visibility
    if (hsl.l > 0.7) {
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
    image.crossOrigin = "anonymous";
    image.src = previousBackground = currentAlbumImage;

    if (!image.complete) {
        await new Promise((resolve, reject) => {
            image.onload = () => resolve(null);
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

    const baseCanvas = getCanvasFromPool(scaledWidth, scaledHeight);
    const baseContext = baseCanvas.getContext("2d", { alpha: false })!;

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

    const newCanvas = await createCanvas({ usePreviousColors: reinitialize });
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
        canvasContext = canvas.getContext("2d")!;
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
    LoadingController.setLoadingBackground(true);
}

// New smooth transition function with interruption support
async function transitionToNewCanvas(force = false) {
    const _newCanvas = await createCanvas({ force });
    if (!_newCanvas) return;

    // Cancel active transition and use its target as new starting point
    if (activeTransition !== null) {
        cancelAnimationFrame(activeTransition);
        activeTransition = null;

        // If there was a pending canvas, use it as current
        if (pendingTransition) {
            if (currentCanvas) {
                returnCanvasToPool(currentCanvas);
            }
            currentCanvas = pendingTransition;
            pendingTransition = null;
        }
    }

    // Set new pending canvas
    if (pendingTransition) {
        returnCanvasToPool(pendingTransition);
    }
    pendingTransition = _newCanvas;

    const width = canvas.width;
    const height = canvas.height;

    // Reuse buffer canvas
    const buffer = getCanvasFromPool(width, height);
    const bufferContext = buffer.getContext("2d", { alpha: false })!;

    const startTime = performance.now();
    let lastProgress = 0;

    function animate(currentTime: number) {
        let elapsed = currentTime - startTime;
        if (elapsed < 0) elapsed = 0;

        // Linear progress
        const progress = Math.min(elapsed / CANVAS_TRANSITION_DURATION, 1);

        // Only update if progress changed significantly (optimization)
        if (Math.abs(progress - lastProgress) < 0.01 && progress < 1) {
            activeTransition = requestAnimationFrame(animate);
            return;
        }
        lastProgress = progress;

        bufferContext.clearRect(0, 0, width, height);

        bufferContext.globalAlpha = 1;
        if (currentCanvas) {
            bufferContext.drawImage(currentCanvas, 0, 0, width, height);
        }

        bufferContext.globalAlpha = progress;
        bufferContext.drawImage(pendingTransition!, 0, 0, width, height);

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
            currentCanvas = pendingTransition;
            pendingTransition = null;
            activeTransition = null;
            returnCanvasToPool(buffer);
        }
    }

    activeTransition = requestAnimationFrame(animate);
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
        transitionToNewCanvas()
    );
    unlistenMusicPlaylist = musicPlaylist.subscribe(() =>
        transitionToNewCanvas()
    );
    unlistenSettingTriggerAnimatedBackground =
        settingTriggerAnimatedBackground.subscribe(() => {
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
    if (pendingTransition) returnCanvasToPool(pendingTransition);

    // Clear pool
    canvasPool.length = 0;

    unlistenMusicCurrentIndex();
    unlistenMusicPlaylist();
    unlistenSettingTriggerAnimatedBackground();
});
</script>

<svelte:window onresize={onWindowResize} />
<canvas class="fixed rounded-windows"
        class:hidden={page.url.pathname === PageRoutes.VISUALIZER}
        bind:this={canvas}></canvas>