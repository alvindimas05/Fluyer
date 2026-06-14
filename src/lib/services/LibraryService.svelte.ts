import musicStore from '$lib/stores/music.svelte';
import TauriLibraryAPI from '$lib/tauri/TauriLibraryAPI';
import PersistentStoreService from './PersistentStoreService.svelte';

const LibraryService = {
	initialize: async () => {
		await LibraryService.loadMusicList();
	},
	loadMusicList: async () => {
		if ((await PersistentStoreService.musicPath.get()).length === 0) {
			musicStore.isLibraryLoaded = false;
			return;
		}

		const now = performance.now();
		await TauriLibraryAPI.sync();
		const counts = await TauriLibraryAPI.load();

		console.log(`Scanning music list took ${performance.now() - now} ms`);

		// Now load the sorted/grouped state into LibraryState and get counts
		musicStore.listCount = counts.musicCount;
		musicStore.albumCount = counts.albumCount;
		musicStore.isLibraryLoaded = true;
	}
};

export default LibraryService;
