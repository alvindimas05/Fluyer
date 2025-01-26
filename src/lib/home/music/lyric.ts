export default class MusicLyric {
	duration = 0;
	lyric = "";
	constructor(text: string) {
		let [timestamp, lyric] = text.split(/(?<=^\S+)\s/);
		const match = timestamp.match(/\[(\d{2}):(\d{2})\.(\d{2})\]/);
		if (match) {
			const [, minutes, seconds, milliseconds] = match.map(Number);
			this.duration = minutes * 60 + seconds + milliseconds / 100;
		}

		this.lyric = lyric.length < 1 ? "..." : lyric;
	}
}
