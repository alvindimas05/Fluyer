import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';

const TauriMetadataAPI = {
	getMusicCoverArt: (path: string, size?: number) => {
		return invoke<ArrayBuffer>(CommandRoutes.MUSIC_GET_IMAGE, { path, size });
	},
	getDefaultCoverArt: () => {
		return invoke<ArrayBuffer>(CommandRoutes.MUSIC_GET_DEFAULT_COVER_ART);
	},
	getFolderCoverArtPath: (path: string) => {
		return invoke<string>(CommandRoutes.FOLDER_GET_FIRST_MUSIC_PATH, { path });
	}
};

export default TauriMetadataAPI;
