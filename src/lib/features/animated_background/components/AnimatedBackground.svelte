<script lang="ts">
	import '../style.scss';
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/state';
	import { PageRoutes } from '$lib/constants/PageRoutes';
	import { isLinux } from '$lib/platform';
	import { afterNavigate } from '$app/navigation';
	import { MusicConfig } from '$lib/constants/MusicConfig';
	import MetadataService from '$lib/services/MetadataService.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import settingStore from '$lib/stores/setting.svelte';
	import LibraryService from '$lib/services/LibraryService.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let currentAlbumImage: string | null = null;
    let isInitialized = false;

	async function updateBackground(force = false) {
		const newAlbumImage = await MetadataService.getMusicCoverArt(musicStore.currentMusic);

		if (currentAlbumImage === newAlbumImage && !force) return;
		currentAlbumImage = newAlbumImage;

        // Pass path to Rust
        // MetadataService.getMusicCoverArt usually returns a URL or blob. 
        // If it returns a file path (custom protocol), we need to extract it.
        // But getMusicCoverArt might return a blob URL if it read the file.
        // We need the original file path if possible.
        // Checking MetadataService...
        
        // Assuming we can get the path from musicStore.currentMusic.cover_art or similar?
        // Let's check musicStore structure or just pass what we have.
        // If it's a blob URL, Rust can't read it easily without fetching it.
        // But `update_animated_background` expects a file path.
        
        let audioPath = musicStore.currentMusic?.path; 
		console.log('Updating background with path:', audioPath);       
		await invoke('update_animated_background', { audioPath }); // Use snake_case explicitly
        
        if (!isInitialized) {
             isInitialized = true;
             // Signal to other components? 
             // "Wait for wgpu background to initialize first them show svelte components"
             // Maybe set a global store value?
             LibraryService.initialize();
             console.log('AnimatedBackground is initialized (WGPU)');
        }
	}

	function onWindowResize() {
		// WGPU handles resize via Rust event
	}

	onMount(() => {
		updateBackground(true);

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

		// $effect(() => {
		// 	settingStore.animatedBackground.trigger;
		// 	console.log('Updating background from trigger');
		// 	updateBackground(true);
		// });
	});

	onDestroy(() => {
	
	});
</script>

<svelte:window onresize={onWindowResize} />
<!-- We don't need a canvas anymore, WGPU renders to the window surface -->
<!-- But we might need a transparent container if we want to ensure pointer events pass through? -->
<div class="fixed inset-0 -z-10 pointer-events-none"></div>
