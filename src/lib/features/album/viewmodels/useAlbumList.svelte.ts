import { isMobile } from '$lib/platform';
import mobileStore from '$lib/stores/mobile.svelte';
import filterStore from '$lib/stores/filter.svelte';
import musicStore from '$lib/stores/music.svelte';
import { type MusicData, MusicListType } from '$lib/features/music/types';
import sidebarStore from '$lib/stores/sidebar.svelte';

const RESPONSIVE_RULES = [
	[1536, 2.01, 0.125], // xhdpi 2xl → 12.5%
	[1280, 2.01, 0.16667], // xl-xhdpi → 16.6667%
	[1024, 2.01, 0.2], // lg-xhdpi → 20%
	[768, 2.01, 0.25], // md-xhdpi → 25%
	[640, 2.01, 0.33334], // sm-xhdpi → 33.3334%
	[1536, 1.01, 0.125], // hdpi 2xl → 12.5%
	[1280, 1.01, 0.16667], // xl-hdpi → 16.6667%
	[1024, 1.01, 0.2], // lg-hdpi → 20%
	[768, 1.01, 0.25], // md-hdpi → 25%
	[640, 1.01, 0.33334], // sm-hdpi → 33.3334%
	[1536, 0, 0.125], // default 2xl → 12.5%
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
			return;
		}
	}

	state.columnCount = 2;
	state.itemWidth = 0.5 * width;
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

export function useAlbumList() {
	$effect(() => {
		sidebarStore.swipeMinimumTop = paddingTop + itemHeight;
	});

	$effect(() => {
		updateItemWidth();
	});

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

		updateItemWidth
	};
}
