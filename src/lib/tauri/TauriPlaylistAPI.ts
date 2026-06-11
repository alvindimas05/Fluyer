import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';
import type { PlaylistData } from '$lib/features/music/types';

const TauriPlaylistAPI = {
	getAll: () => {
		return invoke<PlaylistData[]>(TauriCommands.PLAYLIST_ALL_GET);
	},
	create: (playlist: PlaylistData) => {
		return invoke<number>(TauriCommands.PLAYLIST_CREATE, { playlist });
	},
	delete: (id: number) => {
		return invoke(TauriCommands.PLAYLIST_DELETE, { id });
	},
	uploadImage: () => {
		return invoke<string>(TauriCommands.PLAYLIST_IMAGE_UPLOAD);
	},
	readImage: (id: number) => {
		return invoke<ArrayBuffer>(TauriCommands.PLAYLIST_IMAGE_READ, { id });
	}
};

export default TauriPlaylistAPI;
