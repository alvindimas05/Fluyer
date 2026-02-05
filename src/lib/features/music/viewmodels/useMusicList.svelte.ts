import { MusicListType } from '$lib/features/music/types';
import musicStore from '$lib/stores/music.svelte';
import filterStore from '$lib/stores/filter.svelte';
import folderStore from '$lib/stores/folder.svelte';
import LibraryService from '$lib/services/LibraryService.svelte';
import FolderService from '$lib/services/FolderService.svelte';
import sidebarStore from '$lib/stores/sidebar.svelte';
import { SidebarType } from '$lib/features/sidebar/types';

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
	columnCount: 1,
	scrollTop: 0
});

// Track visibility of items using IntersectionObserver
let visibleItems = $state<Set<string>>(new Set());
let observer: IntersectionObserver | null = null;

// Track items that are animating out (hidden by sidebar)
let animatingOutItems = $state<Set<string>>(new Set());

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

	const filteredMusic = LibraryService.sortMusicList(
		musicStore.list.filter((music) => {
			if (isFolderMode) {
				return FolderService.containsMusic(music, folderStore.currentFolder);
			}
			return true;
		})
	);

	let filteredFolders = folderStore.list;

	if (!filterStore.bar.sortAsc) filteredFolders = [...filteredFolders].reverse();

	const finalList: any[] = filterStore.album
		? LibraryService.sortMusicList(filteredMusic)
		: [...filteredMusic];

	if (!filterStore.bar.sortAsc) finalList.reverse();

	if (isFolderMode) {
		const nonEmpty = filteredFolders.filter((f) => FolderService.getMusicList(f).length > 0);
		finalList.push(...nonEmpty);
	}

	return finalList;
});

function isVisibleByFilter(item: any): boolean {
	const search = filterStore.search.toLowerCase();

	// Folder check
	if (!('duration' in item)) {
		// Folder
		return item.path.toLowerCase().includes(search);
	}

	// Music check
	const music = item;
	const album = filterStore.album;
	const hasSearch = search.length > 0;
	const matchesSearch =
		!hasSearch ||
		[music.album, music.title, music.artist, music.albumArtist].some((v) =>
			v?.toLowerCase().includes(search)
		);

	const hasAlbum = !!album;
	const matchesAlbum = !hasAlbum || album.name === music.album;

	return matchesSearch && matchesAlbum;
}

// Calculate visual indices for items that are visible references
const visualIndices = $derived.by(() => {
	filterStore.search;
	filterStore.album;

	const map = new Map<number, number>();
	let count = 0;

	if (data) {
		data.forEach((item, index) => {
			if (isVisibleByFilter(item)) {
				map.set(index, count++);
			}
		});
	}
	return map;
});

function isHiddenBySidebar(index: number): boolean {
	if (!visualIndices.has(index)) return true; // Should be hidden anyway if not in visual map

	const visualIndex = visualIndices.get(index)!;
	const indexInRow = visualIndex % state.columnCount;

	if (sidebarStore.showType === SidebarType.Left) {
		return indexInRow < sidebarStore.hiddenMusicColumnCount;
	}
	if (sidebarStore.showType === SidebarType.Right) {
		return indexInRow >= state.columnCount - sidebarStore.hiddenMusicColumnCount;
	}
	return false;
}

function getItemKey(item: any): string {
	if ('duration' in item) {
		return `music-${item.path}`;
	}
	return `folder-${item.path}`;
}

// Check if item should render based on all visibility conditions
function shouldRenderItem(itemKey: string, index: number, item: any): boolean {
	// If not visible by filter, don't render
	if (!isVisibleByFilter(item)) return false;

	// If not in visibleItems (outside viewport), don't render
	if (!visibleItems.has(itemKey)) return false;

	// If hidden by sidebar and animation completed, don't render
	if (isHiddenBySidebar(index) && animatingOutItems.has(itemKey)) return false;

	return true;
}

// Handle sidebar fadeout animation completion
function handleAnimationEnd(itemKey: string, isHiddenBySidebar: boolean) {
	if (isHiddenBySidebar) {
		animatingOutItems = new Set([...animatingOutItems, itemKey]);
	}
}

function observeElement(node: HTMLElement, key: string) {
	if (!observer) {
		observer = new IntersectionObserver(
			(entries) => {
				const newVisible = new Set(visibleItems);
				let changed = false;

				entries.forEach((entry) => {
					const itemKey = entry.target.getAttribute('data-item-key');
					if (itemKey) {
						if (entry.isIntersecting) {
							if (!newVisible.has(itemKey)) {
								newVisible.add(itemKey);
								changed = true;
							}
						} else {
							if (newVisible.has(itemKey)) {
								newVisible.delete(itemKey);
								changed = true;
							}
						}
					}
				});

				if (changed) {
					visibleItems = newVisible;
				}
			},
			{ threshold: 0 }
		);
	}

	node.setAttribute('data-item-key', key);
	observer.observe(node);

	return {
		update(newKey: string) {
			node.setAttribute('data-item-key', newKey);
			observer?.unobserve(node);
			observer?.observe(node);
		},
		destroy() {
			observer?.unobserve(node);
		}
	};
}

function handleScroll(e: Event) {
	const target = e.target as HTMLDivElement;
	state.scrollTop = target.scrollTop;
}

export function useMusicList() {
	$effect(() => {
		musicStore.listType;
		updateSize();
	});

	// Reset animating out state when item becomes visible by sidebar
	$effect(() => {
		if (data) {
			data.forEach((item, index) => {
				const itemKey = getItemKey(item);
				if (!isHiddenBySidebar(index) && animatingOutItems.has(itemKey)) {
					animatingOutItems = new Set([...animatingOutItems].filter((k) => k !== itemKey));
				}
			});
		}
	});

	// We can't easily move dom manipulation like setting scrollTop on mount without binding.
	// We'll expose state.scrollTop and let the component bind it or use an action.
	// But the component uses `onscroll`, so we expose `handleScroll`.
	// For restoring scroll, we can use an action or just $effect in component.
	// To keep it clean, let's create a `scrollable` action.

	function scrollable(node: HTMLElement) {
		node.scrollTop = state.scrollTop;
		return {
			destroy() {
				//
			}
		}
	}

	return {
		state,

		get data() {
			return data;
		},

		get visibleItems() {
			return visibleItems;
		},

		updateSize,
		getItemKey,
		shouldRenderItem,
		isVisibleByFilter,
		isHiddenBySidebar,
		handleAnimationEnd,
		observeElement,
		handleScroll,
		scrollable
	};
}
