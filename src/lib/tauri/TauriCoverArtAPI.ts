import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';

const TauriCoverArtAPI = {
	getCoverArt: (query: { artist: string; album?: string; title?: string }, size?: number) => {
		return invoke<number[]>(TauriCommands.COVER_ART_GET, { query, size });
	}
};

export default TauriCoverArtAPI;
