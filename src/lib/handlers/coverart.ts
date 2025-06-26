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
	fromQuery: async (query: CoverArtCacheQuery) => {
		try {
			let cache = MusicController.getCoverArtCache(query);
			if (cache != null) return cache.status;
			MusicController.addCoverArtCache({
				name: `${query.artist} ${query.album ?? query.title ?? ""}`,
				status: CoverArtStatus.Loading,
				image: null,
			});

			if (query.album) query.title = undefined;

			let cover = await invoke<CoverArtResponse>(CommandRoutes.COVER_ART_GET, {
				query,
			});
			MusicController.setCoverArtCache(query, cover);
			return cover.status;
		} catch (err) {
			console.error(err);
			return CoverArtStatus.Failed;
		}
	},
};

export default CoverArt;
