<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import { prominent } from "color.js";
import "./background.scss";
import LoadingController from "$lib/controllers/LoadingController";
import BackgroundController from "$lib/controllers/BackgroundController";
import { musicCurrentIndex } from "$lib/stores/music";
import { onMount } from "svelte";
import * as StackBlur from "stackblur-canvas";

const CANVAS_BLOCK_SIZE = 100;
const CANVAS_TRANSITION_SPEED = 0.03;
const CANVAS_BLUR_RADIUS = 200;

let previousBackground: string | null = null;
let canvas: HTMLCanvasElement;
let canvasContext: CanvasRenderingContext2D;
let currentCanvas: HTMLCanvasElement | null = null;
let newCanvas: HTMLCanvasElement | null = null;

function hexToRgb(hex: string): { r: number; g: number; b: number } {
	const bigint = parseInt(hex.slice(1), 16);
	const r = (bigint >> 16) & 255;
	const g = (bigint >> 8) & 255;
	const b = bigint & 255;
	return { r, g, b };
}

function getLuminance(r: number, g: number, b: number): number {
	const a = [r, g, b].map((v) => {
		v /= 255;
		return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
	});
	return 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2];
}

function isMajorityLight(colors: string[]): boolean {
	let lightCount = 0;
	let darkCount = 0;

	for (const hex of colors) {
		const { r, g, b } = hexToRgb(hex);
		const luminance = getLuminance(r, g, b);
		if (luminance > 0.5) {
			lightCount++;
		} else {
			darkCount++;
		}
	}

	return lightCount > darkCount;
}

async function getColors(): Promise<string[] | null> {
	const currentAlbumImage = MusicController.currentMusicAlbumImage()
	if (
		previousBackground !== null &&
		previousBackground === currentAlbumImage
	)
		return null;

	let image = new Image();
	image.src = previousBackground = currentAlbumImage;

	// @ts-ignore
	let colors: Hex[] = await prominent(image, {
		amount: 10,
		format: "hex",
	});
	BackgroundController.setIsLight(isMajorityLight(colors));
	
	return colors;
}

function createCanvas(colors: string[]): HTMLCanvasElement {
    const canvasBlockSize = CANVAS_BLOCK_SIZE;
    const baseCanvas = document.createElement("canvas");
    baseCanvas.width = canvas.width;
    baseCanvas.height = canvas.height;
    const baseContext = baseCanvas.getContext("2d")!;
    
    for (let y = 0; y < canvas.height; y += canvasBlockSize) {
        for (let x = 0; x < canvas.width; x += canvasBlockSize) {
            const color = colors[Math.floor(Math.random() * colors.length)];
            baseContext.fillStyle = color;
            baseContext.fillRect(x, y, canvasBlockSize, canvasBlockSize);
        }
    }
    
    StackBlur.canvasRGBA(baseCanvas, 0, 0, canvas.width, canvas.height, CANVAS_BLUR_RADIUS);
    
    return baseCanvas;
}

async function initializeCanvas(){
    canvasContext = canvas.getContext("2d")!;
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    currentCanvas = createCanvas((await getColors())!);
    
    let alpha = 0;
    function fadeIn() {
        canvasContext.clearRect(0, 0, canvas.width, canvas.height);
        canvasContext.globalAlpha = alpha;
        canvasContext.drawImage(currentCanvas!, 0, 0);
        canvasContext.globalAlpha = 1.0;
        
        if (alpha < 1) {
            alpha += CANVAS_TRANSITION_SPEED;
            requestAnimationFrame(fadeIn);
        } else {
            afterInitializeCanvas();
        }
    }

    fadeIn();
}

async function afterInitializeCanvas(){
    LoadingController.setLoadingBackground(true);
    musicCurrentIndex.subscribe(() => setTimeout(transitionToNewCanvas, 0));
}

async function transitionToNewCanvas() {
    const colors = await getColors();
    if (!colors) return;

    newCanvas = createCanvas(colors);

    const width = canvas.width;
    const height = canvas.height;

    const buffer = document.createElement("canvas");
    buffer.width = width;
    buffer.height = height;
    const bufferContext = buffer.getContext("2d")!;

    let progress = 0;

    function animate() {
        bufferContext.clearRect(0, 0, width, height);

        bufferContext.globalAlpha = 1;
        bufferContext.drawImage(currentCanvas!, 0, 0, width, height);

        bufferContext.globalAlpha = progress;
        bufferContext.drawImage(newCanvas!, 0, 0, width, height);

        bufferContext.globalAlpha = 1;
        canvasContext.clearRect(0, 0, width, height);
        canvasContext.drawImage(buffer, 0, 0);

        if (progress < 1) {
            progress += CANVAS_TRANSITION_SPEED;
            requestAnimationFrame(animate);
        } else {
            currentCanvas = newCanvas;
            newCanvas = null;
        }
    }

    animate();
}

onMount(() => {
    initializeCanvas();
});
</script>

<canvas class="fixed" bind:this={canvas}></canvas>