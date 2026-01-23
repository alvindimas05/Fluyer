import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';

interface LyricQuery {
	path: string;
	title: string;
	artist: string;
	album?: string;
	duration?: number;
}

const TauriLyricAPI = {
	get: (path: string) => {
		return invoke<string | null>(CommandRoutes.MUSIC_GET_LYRICS, { path });
	},
	search: (query: LyricQuery) => {
		return invoke<string | null>(CommandRoutes.LYRIC_GET, { query });
	}
};

export default TauriLyricAPI;
