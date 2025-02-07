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
	// FIXME: Hangs Rust if it has too many requests
	fromQuery: async (query: CoverArtCacheQuery) => {
		try {
			let cache = MusicController.getCoverArtCache(query);
			if (cache != null) return cache.status;
			MusicController.addCoverArtCache({
				name: `${query.artist} ${query.album ?? query.title ?? ""}`,
				status: CoverArtStatus.Loading,
				image: null,
			});

			let cover = await invoke<CoverArtResponse>(CommandsRoute.COVER_ART_GET, {
				query,
			});
			console.log(cover);
			MusicController.setCoverArtCache(query, cover);
			return cover.status;
		} catch (err) {
			console.error(err);
			return CoverArtStatus.Failed;
		}
	},
};

export default CoverArt;
