import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';

const TauriVisualizerAPI = {
	getMusicBuffer: async (path: string) => {
		return invoke(CommandRoutes.MUSIC_GET_VISUALIZER_BUFFER, { path });
	}
};

export default TauriVisualizerAPI;
