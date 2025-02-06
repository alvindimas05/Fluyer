import MusicBrainzApi from "$lib/api/musicbrainz";
import { CommandsRoute } from "$lib/commands";
import type { MusicData } from "$lib/home/music/types";
import { coverArtAlbumCaches } from "$lib/stores/coverart";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

const mbApi = new MusicBrainzApi();

export enum CoverArtStatus {
    Loaded = "loaded",
    Loading = "loading",
    Failed = "failed",
}
export interface CoverArtResponse {
    name: string,
    status: CoverArtStatus,
    image: string
}


const CoverArt = {
	// FIXME: Instead of reading one file for multiple music/album, add checking and only read one time if it request the same album
	fromAlbum: async (album: string) => {
		try {
			let albumCover = await invoke<CoverArtResponse>(CommandsRoute.COVER_ART_FROM_ALBUM, {
				album,
			});
			if (albumCover.status == CoverArtStatus.Failed) return false;
			if (albumCover.status == CoverArtStatus.Loaded || albumCover.image != null){
				let caches = get(coverArtAlbumCaches);
				caches.push(albumCover);
				coverArtAlbumCaches.set(caches);
			}
			return true;
		} catch (err) {
			console.error(err);
			return false;
		}
	},
	fromMusic: async (music: MusicData) => {
		if (music.album == null) return null;
		return CoverArt.fromAlbum(music.album);
	},
};

export default CoverArt;
