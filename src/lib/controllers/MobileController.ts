import { CommandRoutes } from "$lib/commands";
import { isDesktop } from "$lib/platform";
import {
	mobileNavigationBarHeight,
	mobileShowSwipeGuide,
	mobileStatusBarHeight,
} from "$lib/stores/mobile";
import { invoke } from "@tauri-apps/api/core";
import { PageRoutes } from "$lib/pages";
import { page } from "$app/stores";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";

const MobileController = {
	initialize: async () => {
		if (isDesktop()) return;
		await MobileController.setNavigationBarHeight();
		await MobileController.setStatusBarHeight();
		MobileController.listenPageChange();
		await MobileController.setSwipeGuide();
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
	},
	listenPageChange: () => {
		page.subscribe(async ($page) => {
			const visible = ![PageRoutes.PLAY, PageRoutes.VISUALIZER].includes(
				$page.url.pathname,
			);
			await MobileController.setNavigationBarVisibility(visible);
		});
	},
	setSwipeGuide: async () => {
		mobileShowSwipeGuide.set(await PersistentStoreController.swipeGuide.get());
	},
	hideSwipeGuide: async () => {
		await PersistentStoreController.swipeGuide.set(false);
		mobileShowSwipeGuide.set(false);
	},
};

export default MobileController;
