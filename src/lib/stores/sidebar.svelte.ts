import type { SidebarType } from '$lib/features/sidebar/types';

const sidebarStore = $state({
	showType: null as SidebarType | null,
	swipeMinimumTop: 0,
	hiddenMusicColumnCount: 1, // Number of columns hidden by sidebar (sidebar is 2 columns wide)
	hiddenAlbumColumnCount: 2, // Number of columns hidden by sidebar (sidebar is 2 columns wide)
	width: window.innerWidth,
});

export default sidebarStore;
