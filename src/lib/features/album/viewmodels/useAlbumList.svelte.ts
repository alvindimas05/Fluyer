import { isMobile } from '$lib/platform';
import mobileStore from '$lib/stores/mobile.svelte';
import filterStore from '$lib/stores/filter.svelte';
import musicStore from '$lib/stores/music.svelte';
import { type MusicData, MusicListType } from '$lib/features/music/types';
import sidebarStore from '$lib/stores/sidebar.svelte';
import { SidebarType } from '$lib/features/sidebar/types';
import ToastService from '$lib/services/ToastService.svelte';

const RESPONSIVE_RULES = [
	[1536, 2.01, 0.142857], // xhdpi 2xl → 14.2857%
	[1280, 2.01, 0.16667], // xl-xhdpi → 16.6667%
	[1024, 2.01, 0.2], // lg-xhdpi → 20%
	[768, 2.01, 0.25], // md-xhdpi → 25%
	[640, 2.01, 0.33334], // sm-xhdpi → 33.3334%

	[1536, 1.01, 0.142857], // hdpi 2xl → 14.2857%
	[1280, 1.01, 0.16667], // xl-hdpi → 16.6667%
	[1024, 1.01, 0.2], // lg-hdpi → 20%
	[768, 1.01, 0.25], // md-hdpi → 25%
	[640, 1.01, 0.33334], // sm-hdpi → 33.3334%

	[1536, 0, 0.125], // 2xl → 12.5%
	[1440, 0, 0.142857], // 1440 → 14.2857%
	[1280, 0, 0.16667], // xl → 16.6667%
	[1024, 0, 0.2], // lg → 20%
	[768, 0, 0.25], // md → 25%
	[640, 0, 0.33334] // sm → 33.3334%
];

let state = $state({
	columnCount: 2,
	itemWidth: window.innerWidth * 0.5,
	scrollLeft: 0,
	scrollTop: 0
});

// Track visibility of items using IntersectionObserver
let visibleItems = $state<Set<number>>(new Set());
let observer: IntersectionObserver | null = null;

// Track items that are animating out (hidden by sidebar)
let animatingOutItems = $state<Set<number>>(new Set());

let paddingTop = $derived((isMobile() ? mobileStore.statusBarHeight : 0) + filterStore.bar.height);
let itemHeight = $derived(state.itemWidth + (window.innerWidth > 640 ? 52 : 44));
let isHorizontal = $derived(musicStore.listType !== MusicListType.Album);

function updateItemWidth() {
	const width = window.innerWidth;
	const dpr = window.devicePixelRatio;

	for (const [minWidth, minDppx, widthRatio] of RESPONSIVE_RULES) {
		if (width >= minWidth && dpr >= minDppx) {
			state.itemWidth = widthRatio * width;
			state.columnCount = Math.round(1 / widthRatio);
			sidebarStore.width = state.itemWidth * 2;
			if (state.columnCount === 5 && window.devicePixelRatio < 1.01) {
				sidebarStore.hiddenMusicColumnCount = 2;
				sidebarStore.hiddenAlbumColumnCount = 2;
			} else {
				sidebarStore.hiddenMusicColumnCount = 1;
				sidebarStore.hiddenAlbumColumnCount = 2;
			}
			return;
		}
	}

	state.columnCount = 2;
	state.itemWidth = 0.5 * width;
	sidebarStore.width = window.innerWidth;
	sidebarStore.hiddenAlbumColumnCount = 2;
	sidebarStore.hiddenMusicColumnCount = 1;
}

let data: MusicData[][] = $derived.by(() => {
	if (!Array.isArray(musicStore.albumList)) return [];

	const search = filterStore.search.toLowerCase();
	let list = musicStore.albumList;

	// We return the full list and handle visibility via CSS to prevent component recycling issues
	// when filtering (search or album selection).
	if (!filterStore.bar.sortAsc) list = list.toReversed();

	return list;
});

function isVisibleByFilter(musicList: MusicData[]) {
	const search = filterStore.search.toLowerCase();
	const firstItem = musicList[0];

	if (!filterStore.search && !filterStore.album) return true;

	return (
		(filterStore.album && firstItem.album === filterStore.album.name) ||
		firstItem.album?.toLowerCase().includes(search) ||
		firstItem.albumArtist?.toLowerCase().includes(search)
	);
}

const visualIndices = $derived.by(() => {
	// Reactively depend on filter properties
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

// Calculate sidebar width (2 columns)
let sidebarWidth = $derived(state.itemWidth * sidebarStore.hiddenAlbumColumnCount);
let toastWidth = $derived(state.itemWidth * 2); // User requested 2 items hidden

const extraToleranceWidth = 10;
function shouldHideHorizontalItem(index: number): boolean {
	if (!visualIndices.has(index)) return true;

	const visualIndex = visualIndices.get(index)!;

	// Calculate item's position relative to viewport using visual index
	const itemLeft = visualIndex * state.itemWidth - state.scrollLeft;
	const itemRight = itemLeft + state.itemWidth;
	const viewportWidth = window.innerWidth;

	if (sidebarStore.showType === SidebarType.Left) {
		// Hide if item overlaps with left sidebar area
		if (itemLeft < sidebarWidth - extraToleranceWidth) return true;
	}

	if (sidebarStore.showType === SidebarType.Right) {
		// Hide if item overlaps with right sidebar area
		if (itemRight > viewportWidth - sidebarWidth + extraToleranceWidth) return true;
	}

	// Hide items for Toasts (Right side)
	if (ToastService.toasts.length > 0) {
		if (itemRight > viewportWidth - toastWidth + extraToleranceWidth) return true;
	}

	return false;
}

function shouldHideGridItem(index: number): boolean {
	if (!visualIndices.has(index)) return true;

	const visualIndex = visualIndices.get(index)!;
	const indexInRow = visualIndex % state.columnCount;

	if (sidebarStore.showType === SidebarType.Left) {
		if (indexInRow < sidebarStore.hiddenAlbumColumnCount) return true;
	}
	if (sidebarStore.showType === SidebarType.Right) {
		if (indexInRow >= state.columnCount - sidebarStore.hiddenAlbumColumnCount) return true;
	}

	// Hide items for Toasts (Right side)
	if (ToastService.toasts.length > 0) {
		if (indexInRow >= state.columnCount - 2) return true; // User requested 2 items
	}

	return false;
}

// Check if item should render based on visibility conditions (horizontal)
function shouldRenderHorizontalItem(index: number, musicList: MusicData[]): boolean {
	// If not visible by filter, don't render
	if (!isVisibleByFilter(musicList)) return false;

	// If not in visibleItems (outside viewport), don't render
	if (!visibleItems.has(index)) return false;

	// If hidden by sidebar and animation completed, we still render but hide via CSS
	// if (shouldHideHorizontalItem(index) && animatingOutItems.has(index)) return false;

	return true;
}

// Check if item should render based on visibility conditions (grid)
function shouldRenderGridItem(index: number, musicList: MusicData[]): boolean {
	// If not visible by filter, don't render
	if (!isVisibleByFilter(musicList)) return false;

	// If not in visibleItems (outside viewport), don't render
	if (!visibleItems.has(index)) return false;

	// If hidden by sidebar and animation completed, we still render but hide via CSS
	// if (shouldHideGridItem(index) && animatingOutItems.has(index)) return false;

	return true;
}

// Handle sidebar fadeout animation completion
function handleAnimationEnd(index: number, isHiddenBySidebar: boolean) {
	if (isHiddenBySidebar) {
		animatingOutItems = new Set([...animatingOutItems, index]);
	}
}

function observeElement(node: HTMLElement, index: number) {
	if (!observer) {
		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					const itemIndex = entry.target.getAttribute('data-item-index');
					if (itemIndex !== null) {
						if (entry.isIntersecting) {
							visibleItems = new Set([...visibleItems, parseInt(itemIndex)]);
						}
					}
				});
			},
			{ threshold: 0 }
		);
	}

	node.setAttribute('data-item-index', index.toString());
	observer.observe(node);

	return {
		destroy() {
			observer?.unobserve(node);
		}
	};
}

function handleScroll(e: Event) {
	const target = e.target as HTMLDivElement;
	state.scrollLeft = target.scrollLeft;
	state.scrollTop = target.scrollTop;
}

function handleWheel(e: WheelEvent, scrollContainer: HTMLElement | undefined) {
	if (isHorizontal && e.deltaX === 0) {
		e.preventDefault();
		if (scrollContainer) scrollContainer.scrollLeft += e.deltaY;
	}
}

export function useAlbumList() {
	$effect(() => {
		sidebarStore.swipeMinimumTop = paddingTop + itemHeight;
	});

	$effect(() => {
		const index = musicStore.albumListUi.scrollIndex;
		if (index >= 0) {
			if (isHorizontal) {
				state.scrollLeft = index * state.itemWidth;
			} else {
				const rowIndex = Math.floor(index / state.columnCount);
				state.scrollTop = rowIndex * itemHeight;
			}
			musicStore.albumListUi.scrollIndex = -1;
		}
	});

	$effect(() => {
		updateItemWidth();
	});

	// Reset animating out state when item becomes visible by sidebar
	$effect(() => {
		if (data) {
			data.forEach((_, index) => {
				const isHidden = isHorizontal
					? shouldHideHorizontalItem(index)
					: shouldHideGridItem(index);
				if (!isHidden && animatingOutItems.has(index)) {
					animatingOutItems = new Set([...animatingOutItems].filter((i) => i !== index));
				}
			});
		}
	});

	function scrollable(node: HTMLElement) {
		node.scrollLeft = state.scrollLeft;
		node.scrollTop = state.scrollTop;
		return {
			destroy() { }
		}
	}

	return {
		state,

		get isHorizontal() {
			return isHorizontal;
		},
		get paddingTop() {
			return paddingTop;
		},
		get itemHeight() {
			return itemHeight;
		},
		get data() {
			return data;
		},
		get visibleItems() {
			return visibleItems;
		},

		updateItemWidth,
		isVisibleByFilter,
		shouldHideHorizontalItem,
		shouldHideGridItem,
		shouldRenderHorizontalItem,
		shouldRenderGridItem,
		handleAnimationEnd,
		observeElement,
		handleScroll,
		handleWheel,
		scrollable,
		get filteredItemCount() {
			return visualIndices.size;
		}
	};
}
