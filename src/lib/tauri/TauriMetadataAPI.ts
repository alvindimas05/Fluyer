import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';

const TauriMetadataAPI = {
	getMusicCoverArt: (path: string, size?: number) => {
		return invoke<ArrayBuffer>(TauriCommands.MUSIC_IMAGE_GET, { path, size });
	},
	getDefaultCoverArt: () => {
		return invoke<ArrayBuffer>(TauriCommands.MUSIC_DEFAULT_COVER_ART_GET);
	},
	getFolderCoverArtPath: (path: string) => {
		return invoke<string>(TauriCommands.FOLDER_FIRST_MUSIC_PATH_GET, { path });
	}
};

export default TauriMetadataAPI;
