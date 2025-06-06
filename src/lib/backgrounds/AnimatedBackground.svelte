<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import { prominent } from "color.js";
import "./background.scss";
import LoadingController from "$lib/controllers/LoadingController";
import { musicCurrentIndex } from "$lib/stores/music";
import { onMount } from "svelte";
import * as StackBlur from "stackblur-canvas";
import type { Unsubscriber } from "svelte/store";
import { beforeNavigate } from "$app/navigation";

const CANVAS_BLOCK_SIZE = 150;
const CANVAS_TRANSITION_SPEED = 0.035;
const CANVAS_BLUR_RADIUS = 200;

let previousBackground: string | null = null;
let canvas: HTMLCanvasElement;
let canvasContext: CanvasRenderingContext2D;
let currentCanvas: HTMLCanvasElement | null = null;
let newCanvas: HTMLCanvasElement | null = null;
let unlistenMusicCurrentIndex: Unsubscriber;
let previousColors: string[] = [];
let previousBackgroundColors: string[][] = [];

function hexToRgb(hex: string): { r: number; g: number; b: number } {
	const bigint = parseInt(hex.slice(1), 16);
	const r = (bigint >> 16) & 255;
	const g = (bigint >> 8) & 255;
	const b = bigint & 255;
	return { r, g, b };
}

function darkenTooBright(hex: string, threshold = 0.8): string {
	const { r, g, b } = hexToRgb(hex);
	const luminance = getLuminance(r, g, b);

	if (luminance > threshold) {
		const factor = 0.6;
		const newR = Math.floor(r * factor);
		const newG = Math.floor(g * factor);
		const newB = Math.floor(b * factor);
		return rgbToHex(newR, newG, newB);
	}

	return hex;
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

async function getColors(): Promise<string[] | null> {
	const currentAlbumImage = MusicController.currentMusicAlbumImage();
	if (previousBackground !== null && previousBackground === currentAlbumImage)
		return null;

	let image = new Image();
	image.src = previousBackground = currentAlbumImage;

	// @ts-ignore
	let colors: string[] = await prominent(image, {
		amount: 10,
		format: "hex",
	});

	colors = colors.map((hex) => darkenTooBright(hex));
	return colors;
}

async function createCanvas(options = {
    usePreviousColors: false
}): Promise<HTMLCanvasElement | null> {
    const canvasBlockSize = CANVAS_BLOCK_SIZE;

    if (!options.usePreviousColors) {
        const colors = await getColors();
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
                const color = previousColors[Math.floor(Math.random() * previousColors.length)];
                previousBackgroundColors[y][x] = color;
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
            baseContext.fillRect(x * canvasBlockSize, y * canvasBlockSize, canvasBlockSize, canvasBlockSize);
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
	
	if(reinitialize){
        canvasContext.clearRect(0, 0, canvas.width, canvas.height);
  		canvasContext.drawImage(currentCanvas!, 0, 0);
	} else {
	    canvasContext = canvas.getContext("2d")!;
     
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
}

async function afterInitializeCanvas() {
	LoadingController.setLoadingBackground(true);
	unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(() =>
		setTimeout(transitionToNewCanvas, 0),
	);
}

async function transitionToNewCanvas() {
	const _newCanvas = await createCanvas();

	if(!_newCanvas) return;
	
	newCanvas = _newCanvas;

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

function onWindowResize() {
	initializeCanvas(true);
}

onMount(() => {
	initializeCanvas();
});

beforeNavigate(() => {
	unlistenMusicCurrentIndex();
});
</script>

<svelte:window onresize={onWindowResize} />
<canvas class="fixed" bind:this={canvas}></canvas>