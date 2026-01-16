import { CommandRoutes } from '$lib/constants/CommandRoutes';
import { invoke } from '@tauri-apps/api/core';

const TauriMobileAPI = {
	getNavigationBarHeight: () => {
		return invoke<number>(CommandRoutes.GET_NAVIGATION_BAR_HEIGHT);
	},
	getStatusBarHeight: () => {
		return invoke<number>(CommandRoutes.GET_STATUS_BAR_HEIGHT);
	},
	setNavigationBarVisibility: (visible: boolean) => {
		return invoke<number>(CommandRoutes.SET_NAVIGATION_BAR_VISIBILITY, { visible });
	}
};

export default TauriMobileAPI;
