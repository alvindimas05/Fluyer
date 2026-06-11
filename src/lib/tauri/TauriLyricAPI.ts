import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';

interface LyricQuery {
	path: string;
	title: string;
	artist: string;
	album?: string;
	duration?: number;
}

const TauriLyricAPI = {
	get: (path: string) => {
		return invoke<string | null>(TauriCommands.MUSIC_LYRICS_GET, { path });
	},
	search: (query: LyricQuery) => {
		return invoke<string | null>(TauriCommands.LYRIC_GET, { query });
	}
};

export default TauriLyricAPI;
