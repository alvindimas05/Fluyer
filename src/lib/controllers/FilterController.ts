import { filterAlbum } from "$lib/stores/filter.svelte";
import type { AlbumData } from "$lib/home/music/types";

const FilterController = {
	setFilterAlbum: (value: AlbumData | null) => {
		filterAlbum.set(value);
	},
};

export default FilterController;
