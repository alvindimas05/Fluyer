import { IconType } from '$lib/ui/icon/types';
import { isMacos } from '$lib/platform';
import { type FolderData, MusicListType } from '$lib/features/music/types';
import filterStore from '$lib/stores/filter.svelte';
import { filterBarStore } from '$lib/stores/filter.svelte';
import { IconThemeType } from '$lib/ui/icon/types';
import iconStore from '$lib/stores/icon.svelte';
import folderStore from '$lib/stores/folder.svelte';
import PersistentStoreService from '$lib/services/PersistentStoreService.svelte';
import musicStore from '$lib/stores/music.svelte';
import playlistStore from '$lib/stores/playlist.svelte';
import PlaylistService from '$lib/services/PlaylistService.svelte';
import SidebarService from '$lib/services/SidebarService.svelte';
import { SidebarType } from '$lib/features/sidebar/types';

const RESPONSIVE_RULES = [
	[1536, 2.01, 0.125],
	[1280, 2.01, 0.16667],
	[1024, 2.01, 0.2],
	[768, 2.01, 0.25],
	[640, 2.01, 0.33334],
	[1536, 1.01, 0.125],
	[1280, 1.01, 0.16667],
	[1024, 1.01, 0.2],
	[768, 1.01, 0.25],
	[640, 1.01, 0.33334],
	[1536, 0, 0.125],
	[1280, 0, 0.16667],
	[1024, 0, 0.2],
	[768, 0, 0.25],
	[640, 0, 0.33334]
];

const musicListOptions = [
	{ value: MusicListType.All, icon: IconType.MusicListTypeAll, label: 'All' },
	{ value: MusicListType.Album, icon: IconType.MusicListTypeAlbum, label: 'Album' },
	{ value: MusicListType.Music, icon: IconType.MusicListTypeMusic, label: 'Music' },
	{ value: MusicListType.Folder, icon: IconType.MusicListTypeFolder, label: 'Folder' }
];

let element = $state<HTMLDivElement>();
let state = $state({
	gridSize: ''
});

const iconSize = $derived.by(() => {
	switch (iconStore.theme) {
		case IconThemeType.Phosphor:
			return 19;
		case IconThemeType.Material:
			return 18;
		case IconThemeType.Lucide:
			return 20;
	}
});

function updateGridSizing() {
	const w = window.innerWidth;
	const dpi = window.devicePixelRatio;

	for (const [minW, minDppx, width] of RESPONSIVE_RULES) {
		if (w >= minW && dpi >= minDppx) {
			const columnPercentage = width * window.innerWidth;
			const columns = Math.round(1 / width);
			if (columns > 3) {
				const sideWidth = columnPercentage * 2;
				const restWidth = window.innerWidth - sideWidth * 2;
				state.gridSize = `${sideWidth}px ${restWidth}px ${sideWidth}px`;
			} else if (columns === 3) {
				state.gridSize = `${columnPercentage * 1.5}px 0 ${columnPercentage * 1.5}px`;
			} else {
				state.gridSize = '';
			}
			return;
		}
	}
	state.gridSize = '';
}

function updateFilterBarHeight() {
	if (!element) return;
	filterBarStore.height = element.offsetHeight + (window.innerWidth > 640 ? 8 : 16);
}

function toggleSort() {
	filterBarStore.sortAsc = !filterBarStore.sortAsc;
}

async function handleToggleChange(type: MusicListType) {
	filterStore.album = null;
	folderStore.currentFolder = null;

	// Cancel playlist creation if switching away
	if (playlistStore.isCreating) {
		playlistStore.isCreating = false;
		playlistStore.selectedPaths = [];
	}

	// Set the current folder to the first music path if only one is set
	if (type === MusicListType.Folder) {
		const musicPaths = await PersistentStoreService.musicPath.get();
		folderStore.currentFolder =
			musicPaths.length === 1 ? ({ path: musicPaths[0] } as FolderData) : null;
	}

	musicStore.listType = type;

	console.log('Music list type changed to:', type);
}

function startPlaylistCreation() {
	PlaylistService.requestCreate();
}

function confirmPlaylistCreation() {
	PlaylistService.confirmCreate();
}

function cancelPlaylistCreation() {
	playlistStore.isCreating = false;
	playlistStore.selectedPaths = [];
}

function updateSize() {
	updateGridSizing();
	setTimeout(updateFilterBarHeight);
}

function handleMenuButton() {
	SidebarService.toggle(SidebarType.Left);
}

function handleQueueButton() {
	SidebarService.toggle(SidebarType.Right);
}

export function useFilterBar() {
	return {
		state,
		musicListOptions,
		get element() {
			return element;
		},
		set element(value) {
			element = value;
			updateSize();
		},
		get iconSize() {
			return iconSize;
		},
		toggleSort,
		handleToggleChange,
		updateSize,
		startPlaylistCreation,
		confirmPlaylistCreation,
		cancelPlaylistCreation,
		handleMenuButton,
		handleQueueButton
	};
}
