import { CommandRoutes } from "$lib/constants/CommandRoutes";
import { isAndroid } from "$lib/platform";
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

export enum CoverArtSize {
    Music = 100,
    Album = 400,
    AnimatedBackground = 50,
}

export const COVER_ART_DEBOUNCE_DELAY = isAndroid() ? 1000 : 500;

const CoverArtService = {
    getByQuery: async (
        query: CoverArtCacheQuery,
        size?: CoverArtSize,
    ) => {
        try {
            return (
                await invoke<CoverArtResponse>(CommandRoutes.COVER_ART_GET, {
                    query,
                    size: size?.toString(),
                })
            ).image;
        } catch (err) {
            console.error(err);
            return null;
        }
    },
};

export default CoverArtService;
