import { CommandRoutes } from "$lib/commands";
import MusicController from "$lib/controllers/MusicController";
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
	getImageFromQuery: async (query: CoverArtCacheQuery) => {
		try {
			return (await invoke<CoverArtResponse>(CommandRoutes.COVER_ART_GET, {
				query,
			})).image;
		} catch (err) {
			console.error(err);
			return null;
		}
	}
};

export default CoverArt;
