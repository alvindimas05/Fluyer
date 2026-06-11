import type { AlbumData } from '$lib/features/music/types';

const filterStore = $state({
	search: '',
	album: null as AlbumData | null
});

export default filterStore;

