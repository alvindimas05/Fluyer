import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';

const TauriMetadataAPI = {
	getMusicCoverArt: (path: string) => {
		return invoke<ArrayBuffer>(CommandRoutes.MUSIC_GET_IMAGE, { path });
	},
	getFolderCoverArtPath: (path: string) => {
		return invoke<string>(CommandRoutes.FOLDER_GET_FIRST_MUSIC_PATH, { path });
	}
};

export default TauriMetadataAPI;
