import { MusicListType } from '$lib/features/music/types';
import musicStore from '$lib/stores/music.svelte';
import filterStore from '$lib/stores/filter.svelte';
import folderStore from '$lib/stores/folder.svelte';
import LibraryService from '$lib/services/LibraryService.svelte';
import FolderService from '$lib/services/FolderService.svelte';

const RESPONSIVE_RULES = [
	[1280, 2.01, 4],
	[1024, 2.01, 3],
	[768, 2.01, 2],
	[1536, 1.01, 4],
	[1280, 1.01, 3],
	[768, 1.01, 2],
	[1536, 1.0, 4],
	[1024, 1.0, 3],
	[768, 1.0, 2]
];

const state = $state({
	columnCount: 1
});

function updateColumnCount() {
	const w = window.innerWidth;
	const dpi = window.devicePixelRatio;

	for (const [minW, minDppx, cols] of RESPONSIVE_RULES) {
		if (w >= minW && dpi >= minDppx) {
			state.columnCount = cols;
			return;
		}
	}
	state.columnCount = 1;
}

const updateSize = () => updateColumnCount();

const data = $derived.by(() => {
	if (!Array.isArray(musicStore.list)) return [];

	const isFolderMode = musicStore.listType === MusicListType.Folder;

	// In Folder mode, we only show music in the current folder.
	// We CANNOT filter this via CSS efficiently because the list would be huge (all music files).
	// So we keep folder filtering here.
	const filteredMusic = LibraryService.sortMusicList(
		musicStore.list.filter((music) => {
			if (isFolderMode) {
				return FolderService.containsMusic(music, folderStore.currentFolder);
			}
			return true;
		})
	);

	let filteredFolders = folderStore.list; // Return all folders in current dir?
	// Wait, folderStore.list usually contains folders in current directory.
	// So usually we don't filter them unless search?
	// If we want to hide folders by search in CSS, we return all.

	if (!filterStore.bar.sortAsc) filteredFolders = [...filteredFolders].reverse();

	let finalList: any[] = filterStore.album
		? LibraryService.sortMusicList(filteredMusic)
		: [...filteredMusic];
	// Wait, if filterStore.album is set, we sort by library order?
	// The original code re-sorted if album selected.

	if (!filterStore.bar.sortAsc) finalList.reverse();

	if (isFolderMode) {
		const nonEmpty = filteredFolders.filter((f) => FolderService.getMusicList(f).length > 0);
		finalList.push(...nonEmpty);
	}

	return finalList;
});

export function useMusicList() {
	$effect(() => {
		musicStore.listType;
		updateSize();
	});

	return {
		state,

		get data() {
			return data;
		},

		updateSize
	};
}

