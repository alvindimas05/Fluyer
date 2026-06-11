import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';

const TauriBackgroundAPI = {
	updateBackground: (
		colors: { r: number; g: number; b: number }[] | null,
		width: number,
		height: number
	) => {
		return invoke(TauriCommands.ANIMATED_BACKGROUND_UPDATE, { colors, width, height });
	},
	restoreBackground: () => {
		return invoke(TauriCommands.ANIMATED_BACKGROUND_RESTORE);
	}
};

export default TauriBackgroundAPI;
