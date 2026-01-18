import type { SidebarType } from '$lib/features/sidebar/types';

const sidebarStore = $state({
	showType: null as SidebarType | null,
	swipeMinimumTop: 0,
	hiddenColumnCount: 2 // Number of columns hidden by sidebar (sidebar is 2 columns wide)
});

export default sidebarStore;
