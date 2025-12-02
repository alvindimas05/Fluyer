import {PageRoutes} from "$lib/constants/PageRoutes";
import {page} from "$app/state";
import TauriMobileAPI from "$lib/tauri/TauriMobileAPI";
import mobileStore from "$lib/stores/mobile.svelte";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";

const MobileService = {
    initialize: async () => {
        MobileService.listenPageEvents();
    },
    listenPageEvents: () => {
        $effect(() => {
            const visible = ![PageRoutes.PLAY, PageRoutes.VISUALIZER].includes(
                page.url.pathname,
            );
            TauriMobileAPI.setNavigationBarVisibility(visible);
        });
    },
};

export default MobileService;