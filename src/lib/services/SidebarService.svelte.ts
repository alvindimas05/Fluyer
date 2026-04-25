import type { SidebarType } from '$lib/features/sidebar/types';
import sidebarStore from '$lib/stores/sidebar.svelte';

const SidebarService = {
	show: (type: SidebarType) => {
		sidebarStore.showType = type;
	},
	hide: () => {
		sidebarStore.showType = null;
	},
	toggle: (type: SidebarType) => {
		console.log('toggle', type);
		if (sidebarStore.showType === type) {
			SidebarService.hide();
		} else {
			SidebarService.show(type);
		}
	}
};

export default SidebarService;
