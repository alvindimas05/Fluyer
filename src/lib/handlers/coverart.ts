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

export interface CoverArtCacheQuery {
    artist: string;
    album?: string;
    title?: string;
}

const CoverArt = {
	fromAlbum: async (album: string, artist: string) => {
		try {
		    let albumCache = MusicController.getCoverArtCache({ artist, album });
		    if(albumCache != null) return albumCache.status;
		    MusicController.addCoverArtCache({
				name: `${artist} ${album}`,
				status: CoverArtStatus.Loading,
				image: null
			});
		    
			let albumCover = await invoke<CoverArtResponse>(
				CommandsRoute.COVER_ART_FROM_ALBUM,
				{
					album, artist
				},
			);
			MusicController.setCoverArtCache({ artist, album }, albumCover);
			return albumCover.status;
		} catch (err) {
			console.error(err);
			return CoverArtStatus.Failed;
		}
	},
	fromMusic: async (music: MusicData) => {
		if (music.album != null && music.artist != null){
		    return CoverArt.fromAlbum(music.album, music.artist);
		}
		
		if(music.title == null || music.artist == null) return CoverArtStatus.Failed;
	},
};

export default CoverArt;
