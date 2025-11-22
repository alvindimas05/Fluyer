import type {SidebarType} from "$lib/features/sidebar/types";

const sidebarStore = $state({
    showType: null as SidebarType | null,
    swipeMinimumTop: 0,
});

export default sidebarStore;