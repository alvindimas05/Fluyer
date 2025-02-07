import MusicBrainzApi from "$lib/api/musicbrainz";
import { CommandsRoute } from "$lib/commands";
import MusicController from "$lib/controllers/MusicController";
import type { MusicData } from "$lib/home/music/types";
import { invoke } from "@tauri-apps/api/core";

export enum CoverArtStatus {
	Loaded = "loaded",
	Loading = "loading",
	Failed = "failed",
}
export interface CoverArtResponse {
	name: string;
	status: CoverArtStatus;
	image: string | null;
}

const CoverArt = {
	fromAlbum: async (album: string) => {
		try {
		    let albumCache = MusicController.getCoverArtAlbumCache(album);
		    if(albumCache != null) return albumCache.status;
		    MusicController.addCoverArtAlbumCache({
				name: album,
				status: CoverArtStatus.Loading,
				image: null
			});
		    
			let albumCover = await invoke<CoverArtResponse>(
				CommandsRoute.COVER_ART_FROM_ALBUM,
				{
					album,
				},
			);
			MusicController.setCoverArtAlbumCache(album, albumCover);
			return albumCover.status;
		} catch (err) {
			console.error(err);
			return CoverArtStatus.Failed;
		}
	},
	fromMusic: async (music: MusicData) => {
		if (music.album == null) return null;
		return CoverArt.fromAlbum(music.album);
	},
};

export default CoverArt;
