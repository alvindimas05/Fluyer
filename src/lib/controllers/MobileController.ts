import { CommandsRoute } from "$lib/commands";
import { isDesktop } from "$lib/platform";
import { mobileNavigationBarHeight, mobileStatusBarHeight } from "$lib/stores/mobile";
import { invoke } from "@tauri-apps/api/core";

const MobileController = {
    initialize: () => {
        if(isDesktop()) return;
        MobileController.setNavigationBarHeight();
        MobileController.setStatusBarHeight();
    },
    setStatusBarHeight: async () => {
		const barHeight = await invoke<number>(CommandsRoute.GET_STATUS_BAR_HEIGHT);
		mobileStatusBarHeight.set(barHeight);
	},
	setNavigationBarHeight: async () => {
		const barHeight = await invoke<number>(
			CommandsRoute.GET_NAVIGATION_BAR_HEIGHT,
		);
		mobileNavigationBarHeight.set(barHeight);
	},
}

export default MobileController;