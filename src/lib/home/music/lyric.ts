export default class MusicLyric {
	duration = 0;
	value = "";
	constructor(text: string | null) {
		if (!text) return;

		const match = text.match(/\[(\d{2}):(\d{2})\.(\d{2})\](.*)/);
		if (match) {
			const [, minutes, seconds, milliseconds, lyric] = match;
			this.duration =
				parseInt(minutes) * 60 +
				parseInt(seconds) +
				parseInt(milliseconds) / 100;
			this.value = lyric.trim();
		}
	}
}
