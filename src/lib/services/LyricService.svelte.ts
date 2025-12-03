import type {MusicData} from "$lib/features/music/types";
import TauriLyricAPI from "$lib/tauri/TauriLyricAPI";
import LrcLib from "$lib/api/LrcLib";

const LYRIC_THRESHOLD_DURATION = 5;
export class MusicLyric {
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

const LyricService = {
    get: async (music: MusicData | undefined) => {
        if(!music) return null;

        const lyricPath = music.path.replace(/\.[^/.]+$/, ".lrc");
        try {
            const lyricText = await TauriLyricAPI.get(lyricPath);
            if (lyricText) return LyricService.parse(lyricText);
        } catch (e) {
            console.error(e);
        }

        const lrcText = await LrcLib.getLyrics(music);
        if (!lrcText) return null;

        return LyricService.parse(lrcText);
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

export default LyricService;