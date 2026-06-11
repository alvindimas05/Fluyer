import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';
import type { MusicData } from '$lib/features/music/types';

const TauriLibraryAPI = {
	getMusicList: async () => {
		return invoke<MusicData[] | null>(TauriCommands.MUSIC_ALL_GET);
	}
};

export default TauriLibraryAPI;
