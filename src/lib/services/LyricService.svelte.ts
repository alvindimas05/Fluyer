import type { MusicData } from '$lib/features/music/types';
import TauriLyricAPI from '$lib/tauri/TauriLyricAPI';

const LYRIC_THRESHOLD_DURATION = 5;
export class MusicLyric {
	duration = 0;
	value = '';
	constructor(text: string | null) {
		if (!text) return;

		const match = text.match(/\[(\d{2}):(\d{2})\.(\d{2})\](.*)/);
		if (match) {
			const [, minutes, seconds, milliseconds, lyric] = match;
			this.duration = parseInt(minutes) * 60 + parseInt(seconds) + parseInt(milliseconds) / 100;
			this.value = lyric.trim();
		}
	}
}

const LyricService = {
	get: async (music: MusicData | undefined) => {
		if (!music || !music.title) return null;

		try {
			// Rust backend handles priority: .lrc file → embedded → cache → LrcLib API
			const lrcText = await TauriLyricAPI.search({
				path: music.path,
				title: music.title,
				artist: music.artist || '',
				album: music.album,
				duration: music.duration
			});
			if (lrcText) return LyricService.parse(lrcText);
		} catch (e) {
			console.error('Failed to fetch lyrics:', e);
		}

		return null;
	},
	parse: (text: string) => {
		const rawLyrics = text.split('\n');
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
	}
};

export default LyricService;
