import MusicLyric from "$lib/home/music/lyric";
import type { MusicData } from "$lib/home/music/types";
import axios from "axios";

const LYRIC_THRESHOLD_DURATION = 5;
const LrcLib = {
	getLyrics: async (music: MusicData) => {
		try {
			if (music.title === null) return null;
			let url = new URL("https://lrclib.net/api/search");
			url.searchParams.append(
				"q",
				`${music.title} ${music.artist ?? ""}`.trim(),
			);
			const res = await axios.get<any[]>(url.toString());
			if (
				res.data.length < 1 ||
				res.data[0]["syncedLyrics"] == null ||
				// Make sure the track name is not detected as album name
				!(res.data[0]["name"] as string)
					.toLowerCase()
					.includes(music.title.toLowerCase())
			)
				return null;

			const rawLyrics = (res.data[0]["syncedLyrics"] as string).split("\n");
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
		} catch (err) {
			console.error(err);
		}
		return null;
	},
};

export default LrcLib;
