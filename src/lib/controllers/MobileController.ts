import { CommandRoutes } from "$lib/commands";
import { isDesktop } from "$lib/platform";
import {
	mobileNavigationBarHeight,
	mobileStatusBarHeight,
} from "$lib/stores/mobile";
import { invoke } from "@tauri-apps/api/core";
import {PageRoutes} from "$lib/pages";
import {page} from "$app/stores";

const MobileController = {
	initialize: async () => {
		if (isDesktop()) return;
		await MobileController.setNavigationBarHeight();
		await MobileController.setStatusBarHeight();
		page.subscribe(async ($page) => {
			const visible = $page.url.pathname !== PageRoutes.PLAY;
			await MobileController.setNavigationBarVisibility(visible);
		});
	},
	setStatusBarHeight: async () => {
		const barHeight = await invoke<number>(CommandRoutes.GET_STATUS_BAR_HEIGHT);
		mobileStatusBarHeight.set(barHeight);
	},
	setNavigationBarHeight: async () => {
		const barHeight = await invoke<number>(
			CommandRoutes.GET_NAVIGATION_BAR_HEIGHT,
		);
		mobileNavigationBarHeight.set(barHeight);
	},
	setNavigationBarVisibility: async (visible: boolean) => {
		await invoke(CommandRoutes.SET_NAVIGATION_BAR_VISIBILITY, { visible });
	}
};

export default MobileController;
