import type { MusicData } from "$lib/home/music/types";
import LrcLib from "$lib/api/lrclib";
import MusicLyric from "$lib/home/music/lyric";
import { invoke } from "@tauri-apps/api/core";
import { CommandRoutes } from "$lib/commands";

const LYRIC_THRESHOLD_DURATION = 5;

const LyricController = {
	get: async (music: MusicData) => {
		const lyricPath = music.path.replace(/\.[^/.]+$/, ".lrc");
		try {
			const lyricText = await invoke<string | null>(
				CommandRoutes.MUSIC_GET_LYRICS,
				{ path: lyricPath },
			);
			if (lyricText) return LyricController.parse(lyricText);
		} catch (e) {
			console.error(e);
		}

		const lrcText = await LrcLib.getLyrics(music);
		if (!lrcText) return null;

		return LyricController.parse(lrcText);
	},
	parse: (text: string) => {
		const rawLyrics = text.split("\n");
		let lyrics: MusicLyric[] = [];

		rawLyrics.forEach((lyric, index) => {
			const currentLyric = new MusicLyric(lyric);

			if (index === 0) {
				if (currentLyric.duration > LYRIC_THRESHOLD_DURATION) {
					const emptyLyric = new MusicLyric(null);
					lyrics.push(emptyLyric);
				}

				lyrics.push(currentLyric);
				return;
			}

			lyrics.push(currentLyric);
		});
		return lyrics;
	},
};

export default LyricController;
