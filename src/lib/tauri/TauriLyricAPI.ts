import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';

const TauriLyricAPI = {
	get: (path: string) => {
		return invoke<string | null>(CommandRoutes.MUSIC_GET_LYRICS, { path });
	}
};

export default TauriLyricAPI;
