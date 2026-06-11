import { TauriCommands } from '$lib/constants/TauriCommands';
import { invoke } from '@tauri-apps/api/core';

const TauriMobileAPI = {
	getNavigationBarHeight: () => {
		return invoke<number>(TauriCommands.NAVIGATION_BAR_HEIGHT_GET);
	},
	getStatusBarHeight: () => {
		return invoke<number>(TauriCommands.STATUS_BAR_HEIGHT_GET);
	},
	setNavigationBarVisibility: (visible: boolean) => {
		return invoke<number>(TauriCommands.NAVIGATION_BAR_VISIBILITY_SET, { visible });
	}
};

export default TauriMobileAPI;
