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

	let isInitialized = false;
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
		if(!canUpdate) return;
		if(!isInitialized){
			console.log('AnimatedBackground is initializing...');
			canUpdate = false;
		}
		
		const newCoverArt = await MetadataService.getMusicCoverArt(musicStore.currentMusic);

        const currentWidth = window.innerWidth;
        const currentHeight = window.innerHeight;

		if (currentCoverArt === newCoverArt && !force) return;

		if (currentCoverArt !== null && !MetadataService.isDefaultCoverArt(currentCoverArt)){
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
			 setTimeout(() => canUpdate = true, 1000);

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

	onMount(async () => {
		updateBackground(true);
		if (isAndroid()) unlistenFocus = await listen('tauri://focus', restoreBackground);
	});
	onDestroy(() => unlistenFocus && unlistenFocus());
</script>

<svelte:window onresize={onWindowResize} />
<!-- We don't need a canvas anymore, WGPU renders to the window surface -->
<!-- But we might need a transparent container if we want to ensure pointer events pass through? -->
<div class="fixed inset-0 -z-10 pointer-events-none"></div>
