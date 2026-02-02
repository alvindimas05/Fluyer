<script lang="ts">
	import '../style.scss';
	import { onDestroy, onMount } from 'svelte';
	import { PageRoutes } from '$lib/constants/PageRoutes';
	import { isAndroid, isLinux } from '$lib/platform';
	import { afterNavigate } from '$app/navigation';
	import MetadataService from '$lib/services/MetadataService.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import LibraryService from '$lib/services/LibraryService.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import settingStore from '$lib/stores/setting.svelte';
	import { SettingAnimatedBackgroundType } from '$lib/features/settings/animated_background/types';
	// @ts-ignore
	import ColorThief from 'colorthief/dist/color-thief.mjs';
	import { prominent } from 'color.js';
	import { CommandRoutes } from '$lib/constants/CommandRoutes';
	import { listen } from '@tauri-apps/api/event';
	import type { Unsubscriber } from 'svelte/store';

	interface Color {
		r: number;
		g: number;
		b: number;
	}

	let isInitialized = $state(false);
	let canUpdate = true;
	let currentCoverArt: string | null = null;

	let lastRenderedWidth = 0;
	let lastRenderedHeight = 0;

	let unlistenFocus: Unsubscriber;

	function hexToRgb(hex: string): Color {
		const bigint = parseInt(hex.slice(1), 16);
		const r = (bigint >> 16) & 255;
		const g = (bigint >> 8) & 255;
		const b = bigint & 255;
		return { r, g, b };
	}

	function balanceColor(hex: string): Color {
		const { r, g, b } = hexToRgb(hex);
		const hsl = rgbToHsl(r, g, b);

		// Balance lightness for better visibility
		if (hsl.l > 0.65) {
			// hsl.l = 0.45 + (hsl.l - 0.7) * 0.3;
			hsl.l = 0.65;
		}

		// Ensure we stay within reasonable bounds
		if (MetadataService.isDefaultCoverArt(currentCoverArt)) {
			hsl.s = 0.6;
			hsl.l = 0.6;
		} else {
			// hsl.l = Math.max(0.1, Math.min(0.7, hsl.l));
		}

		return hslToRgb(hsl.h, hsl.s, hsl.l);
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

	function hslToRgb(h: number, s: number, l: number): Color {
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

	async function getColors(): Promise<Color[] | null> {
		if (!currentCoverArt) return null;
		let image = new Image();
		image.crossOrigin = 'anonymous';
		image.src = currentCoverArt;

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

		return colors.map((hex) => balanceColor(hex));
	}

	async function updateBackground(force = false) {
		if (!canUpdate) return;
		if (!isInitialized) {
			console.log('AnimatedBackground is initializing...');
			canUpdate = false;
		}

		const newCoverArt = await MetadataService.getMusicCoverArt(musicStore.currentMusic);

		const currentWidth = window.innerWidth;
		const currentHeight = window.innerHeight;

		if (currentCoverArt === newCoverArt && !force) return;

		if (currentCoverArt !== null && !MetadataService.isDefaultCoverArt(currentCoverArt)) {
			URL.revokeObjectURL(currentCoverArt);
			currentCoverArt = null;
		}

		currentCoverArt = newCoverArt;

		await invoke(CommandRoutes.UPDATE_ANIMATED_BACKGROUND, {
			colors: await getColors(),
			width: currentWidth,
			height: currentHeight
		});

		lastRenderedWidth = currentWidth;
		lastRenderedHeight = currentHeight;

		if (!isInitialized) {
			isInitialized = true;
			// Note: Why? To prevent updateBackground from being called multiple times
			// Since the effects references multiple stores
			setTimeout(() => (canUpdate = true), 1000);

			// Signal to other components?
			// "Wait for wgpu background to initialize first them show svelte components"
			// Maybe set a global store value?
			LibraryService.initialize();
			console.log('AnimatedBackground is initialized (WGPU)');
		}
	}

	function onWindowResize() {
		// Calculate percentage difference
		// The user want to re-render when the window is resized 25% difference
		// Detects from either width or height

		if (lastRenderedWidth === 0 || lastRenderedHeight === 0) {
			lastRenderedWidth = window.innerWidth;
			lastRenderedHeight = window.innerHeight;
			return;
		}

		const widthDiff = Math.abs(window.innerWidth - lastRenderedWidth) / lastRenderedWidth;
		const heightDiff = Math.abs(window.innerHeight - lastRenderedHeight) / lastRenderedHeight;

		if (widthDiff >= 0.25 || heightDiff >= 0.25) {
			console.log('Resized by 25%, updating background');
			updateBackground(true);
		}
	}

	if (isLinux())
		afterNavigate((navigation) => {
			if (navigation.from?.route.id !== PageRoutes.VISUALIZER) return;
			updateBackground(true);
		});

	$effect(() => {
		musicStore.currentIndex;
		musicStore.list;
		console.log('Updating background from effect');
		updateBackground();
	});

	$effect(() => {
		settingStore.animatedBackground.trigger;
		console.log('Updating background from trigger');
		updateBackground(true);
	});

	async function restoreBackground() {
		if (!isInitialized) return;
		await invoke(CommandRoutes.RESTORE_ANIMATED_BACKGROUND);
	}

	// WebGL State
	let canvas = $state<HTMLCanvasElement>();
	let gl: WebGLRenderingContext | null = null;
	let textureCurrent: WebGLTexture | null = null;
	let textureNext: WebGLTexture | null = null;
	let program: WebGLProgram | null = null;
	let unlistenLinuxUpdate: Unsubscriber;

	// Animation State
	let transitionStartTime = 0;
	let isTransitioning = false;
	const TRANSITION_DURATION = 500; // ms

	function setupWebGL() {
		if (!canvas) return;
		gl = canvas.getContext('webgl');
		if (!gl) {
			console.error('WebGL not supported');
			return;
		}

		const vsSource = `
			attribute vec2 position;
			attribute vec2 texCoord;
			varying vec2 vTexCoord;
			void main() {
				gl_Position = vec4(position, 0.0, 1.0);
				vTexCoord = texCoord;
			}
		`;

		const fsSource = `
			precision mediump float;
			uniform sampler2D uImage0;
			uniform sampler2D uImage1;
			uniform float uMix;
			varying vec2 vTexCoord;
			void main() {
				vec4 color0 = texture2D(uImage0, vTexCoord);
				vec4 color1 = texture2D(uImage1, vTexCoord);
				gl_FragColor = mix(color0, color1, uMix);
			}
		`;

		const createShader = (gl: WebGLRenderingContext, type: number, source: string) => {
			const shader = gl.createShader(type);
			if (!shader) return null;
			gl.shaderSource(shader, source);
			gl.compileShader(shader);
			if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
				console.error(gl.getShaderInfoLog(shader));
				gl.deleteShader(shader);
				return null;
			}
			return shader;
		};

		const vs = createShader(gl, gl.VERTEX_SHADER, vsSource);
		const fs = createShader(gl, gl.FRAGMENT_SHADER, fsSource);
		if (!vs || !fs) return;

		program = gl.createProgram();
		if (!program) return;
		gl.attachShader(program, vs);
		gl.attachShader(program, fs);
		gl.linkProgram(program);

		if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
			console.error(gl.getProgramInfoLog(program));
			return;
		}

		gl.useProgram(program);

		// Buffers
		const positionBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		gl.bufferData(
			gl.ARRAY_BUFFER,
			new Float32Array([-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0]),
			gl.STATIC_DRAW
		);

		const positionLoc = gl.getAttribLocation(program, 'position');
		gl.enableVertexAttribArray(positionLoc);
		gl.vertexAttribPointer(positionLoc, 2, gl.FLOAT, false, 0, 0);

		const texCoordBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, texCoordBuffer);
		gl.bufferData(
			gl.ARRAY_BUFFER,
			new Float32Array([0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0]),
			gl.STATIC_DRAW
		);

		const texCoordLoc = gl.getAttribLocation(program, 'texCoord');
		gl.enableVertexAttribArray(texCoordLoc);
		gl.vertexAttribPointer(texCoordLoc, 2, gl.FLOAT, false, 0, 0);

		// Set uniforms for texture units
		const uImage0Loc = gl.getUniformLocation(program, 'uImage0');
		const uImage1Loc = gl.getUniformLocation(program, 'uImage1');
		gl.uniform1i(uImage0Loc, 0);
		gl.uniform1i(uImage1Loc, 1);
	}

	function createTexture(gl: WebGLRenderingContext): WebGLTexture | null {
		const texture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, texture);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
		return texture;
	}

	interface BackgroundUpdatePayload {
		width: number;
		height: number;
		data: number[];
	}

	function updateWebGL(payload: BackgroundUpdatePayload) {
		if (!gl || !program || !canvas) return;

		// Resize canvas if needed
		if (canvas.width !== payload.width || canvas.height !== payload.height) {
			canvas.width = payload.width;
			canvas.height = payload.height;
			gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
		}

		if (!textureCurrent) {
			textureCurrent = createTexture(gl);
		}
		if (!textureNext) {
			textureNext = createTexture(gl);
		}

		const data = new Uint8Array(payload.data);

		// If this is the very first frame, just load it into textureCurrent and draw
		if (!isInitialized) {
			gl.activeTexture(gl.TEXTURE0);
			gl.bindTexture(gl.TEXTURE_2D, textureCurrent);
			gl.texImage2D(
				gl.TEXTURE_2D,
				0,
				gl.RGBA,
				payload.width,
				payload.height,
				0,
				gl.RGBA,
				gl.UNSIGNED_BYTE,
				data
			);

			// Also update textureNext to match, just in case
			gl.activeTexture(gl.TEXTURE1);
			gl.bindTexture(gl.TEXTURE_2D, textureNext);
			gl.texImage2D(
				gl.TEXTURE_2D,
				0,
				gl.RGBA,
				payload.width,
				payload.height,
				0,
				gl.RGBA,
				gl.UNSIGNED_BYTE,
				data
			);

			const uMixLoc = gl.getUniformLocation(program, 'uMix');
			gl.uniform1f(uMixLoc, 0.0);
			gl.drawArrays(gl.TRIANGLES, 0, 6);
			return;
		}

		// Prepare for transition
		// 1. Load new image into textureNext (unit 1)
		gl.activeTexture(gl.TEXTURE1);
		gl.bindTexture(gl.TEXTURE_2D, textureNext);
		gl.texImage2D(
			gl.TEXTURE_2D,
			0,
			gl.RGBA,
			payload.width,
			payload.height,
			0,
			gl.RGBA,
			gl.UNSIGNED_BYTE,
			data
		);

		// 2. Start animation loop
		if (!isTransitioning) {
			isTransitioning = true;
			transitionStartTime = performance.now();
			requestAnimationFrame(animateTransition);
		} else {
			// If already transitioning, we might want to restart or just update the target?
			// For simplicity, let's just update textureNext and reset start time to blend new image
			transitionStartTime = performance.now();
		}
	}

	function animateTransition(time: number) {
		if (!gl || !program) return;

		const elapsed = time - transitionStartTime;
		let mix = Math.min(elapsed / TRANSITION_DURATION, 1.0);

		const uMixLoc = gl.getUniformLocation(program, 'uMix');
		gl.uniform1f(uMixLoc, mix);

		gl.drawArrays(gl.TRIANGLES, 0, 6);

		if (mix < 1.0) {
			requestAnimationFrame(animateTransition);
		} else {
			// Transition finished
			isTransitioning = false;

			// Swap textures logic:
			// Copy textureNext content to textureCurrent effectively by swapping internal handles?
			// Or just ping-pong the uniforms?
			// Easier: Copy next to current for next frame, or just swap variable references and re-bind.

			// Let's swap the references and re-bind to correct units for the "idle" state
			// Idle state: uMix = 0.0, implying we see Texture 0.
			// So we need Texture 0 to contain the "new" image (which is currently in textureNext/Texture 1).

			const temp = textureCurrent;
			textureCurrent = textureNext;
			textureNext = temp;

			// Re-bind to correct units
			gl.activeTexture(gl.TEXTURE0);
			gl.bindTexture(gl.TEXTURE_2D, textureCurrent);

			gl.activeTexture(gl.TEXTURE1);
			gl.bindTexture(gl.TEXTURE_2D, textureNext); // Bind old as next, ready to be overwritten

			// Reset mix to 0
			gl.uniform1f(uMixLoc, 0.0);
			gl.drawArrays(gl.TRIANGLES, 0, 6);
		}
	}

	onMount(async () => {
		updateBackground(true);
		if (isAndroid()) unlistenFocus = await listen('tauri://focus', restoreBackground);

		if (isLinux()) {
			// Initialize WebGL
			// Wait for canvas binding
			setTimeout(async () => {
				setupWebGL();
				unlistenLinuxUpdate = await listen<BackgroundUpdatePayload>(
					'animated_background_update',
					(event) => {
						console.log(
							'Received Linux background update',
							event.payload.width,
							event.payload.height
						);
						updateWebGL(event.payload);
					}
				);
			}, 0);
		}
	});

	onDestroy(() => {
		if (unlistenFocus) unlistenFocus();
		if (unlistenLinuxUpdate) unlistenLinuxUpdate();
	});
</script>

<svelte:window onresize={onWindowResize} />
{#if isLinux()}
	<canvas
		bind:this={canvas}
		class="pointer-events-none fixed inset-0 -z-10 h-full w-full rounded-xl transition-opacity duration-1000 ease-in-out"
		class:opacity-100={isInitialized}
		class:opacity-0={!isInitialized}
	></canvas>
{:else}
	<!-- We don't need a canvas anymore, WGPU renders to the window surface -->
	<!-- But we might need a transparent container if we want to ensure pointer events pass through? -->
	<div class="pointer-events-none fixed inset-0 -z-10"></div>
{/if}
