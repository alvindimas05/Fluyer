import MusicLyric from "$lib/home/music/lyric";
import type { MusicData } from "$lib/home/music/types";
import axios from "axios";

const API = {
	getLyrics: async (music: MusicData) => {
		try {
			let url = new URL("https://lrclib.net/api/search");
			url.searchParams.append(
				"q",
				`${music.title} ${music.artist ?? ""}`.trim(),
			);
			const res = await axios.get<any[]>(url.toString());
			if (res.data.length < 1) return null;

			const rawLyrics = (res.data[0]["syncedLyrics"] as string).split("\n");
			let lyrics: MusicLyric[] = [];

			rawLyrics.forEach((lyric) => {
				lyrics.push(new MusicLyric(lyric));
			});
			return lyrics;
		} catch (err) {
			console.error(err);
		}
		return null;
	},
};

export default API;
