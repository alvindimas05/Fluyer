import type { AlbumData } from '$lib/features/music/types';

const filterStore = $state({
	search: '',
	album: null as AlbumData | null
});

export const filterBarStore = $state({
	height: 0,
	sortAsc: true
});

export default filterStore;
