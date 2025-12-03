import {PageRoutes} from "$lib/constants/PageRoutes";
import {page} from "$app/state";
import TauriMobileAPI from "$lib/tauri/TauriMobileAPI";
import {isAndroid} from "$lib/platform";

const MobileService = {
    initialize: async () => {
        if(!isAndroid()) return;
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