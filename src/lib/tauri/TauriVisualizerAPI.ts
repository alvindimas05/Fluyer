import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';

const TauriVisualizerAPI = {
	getMusicBuffer: async (path: string) => {
		return invoke(TauriCommands.MUSIC_VISUALIZER_BUFFER_GET, { path });
	}
};

export default TauriVisualizerAPI;
