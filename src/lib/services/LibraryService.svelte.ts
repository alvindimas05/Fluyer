import musicStore from '$lib/stores/music.svelte';
import TauriLibraryAPI from '$lib/tauri/TauriLibraryAPI';

const LibraryService = {
	initialize: async () => {
		await LibraryService.loadMusicList();
	},
	loadMusicList: async () => {
		const now = performance.now();
		await TauriLibraryAPI.getMusicList();

		console.log(`Scanning music list took ${performance.now() - now} ms`);

		// Now load the sorted/grouped state into LibraryState and get counts
		const counts = await TauriLibraryAPI.load();
		musicStore.listCount = counts.musicCount;
		musicStore.albumCount = counts.albumCount;
		musicStore.isLibraryLoaded = true;
	}
};

export default LibraryService;
