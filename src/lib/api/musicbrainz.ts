import type { MusicData } from "$lib/home/music/types";
import axios from "axios";

type IType = "recording" | "release";

export default class MusicBrainzApi {
    baseUrl = "https://musicbrainz.org/ws/2";
    baseCoverArtUrl = "https://coverartarchive.org/release";
    appName = import.meta.env.VITE_APP_NAME;
    appVersion = import.meta.env.VITE_APP_VERSION;
    appContactInfo = import.meta.env.VITE_APP_CONTACT_INFO;

    get userAgent() {
        return `${this.appName}/${this.appVersion} ( ${this.appContactInfo} )`;
    }

    get config() {
        return {
            headers: {
                "User-Agent": this.userAgent,
            },
        };
    }

    async browse(browseType: IType, query: string) {
        return await axios.get(
            `${this.baseUrl}/${browseType}?query=${query}&fmt=json&limit=1`,
            this.config,
        );
    }

    async getAlbumImageFromMusic(music: MusicData): Promise<string | null> {
        try {
            let query = "";
            if (music.title) query += ` ${music.title}`;
            if (music.album) query += ` ${music.album}`;
            if (music.artist) query += ` ${music.artist.replaceAll("||", " ")}`;

            const musicRes = await this.browse("release", query);
            const id = musicRes.data.releases[0].id;
            const coverArt = await axios.get(`${this.baseCoverArtUrl}/${id}`);
            return coverArt.data.images[0].image as string;
        } catch (e) {
            console.error("There is an error when getting the image.");
            return null;
        }
    }
}
