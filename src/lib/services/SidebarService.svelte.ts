import type { SidebarType } from "$lib/features/sidebar/types";
import sidebarStore from "$lib/stores/sidebar.svelte";

const SidebarService = {
    show: (type: SidebarType) => {
        sidebarStore.showType = type;
    },
    hide: () => {
        sidebarStore.showType = null;
    },
    toggle: (type: SidebarType) => {
        if (sidebarStore.showType === type) {
            sidebarStore.showType = null;
        } else {
            sidebarStore.showType = type;
        }
    },
};

export default SidebarService;