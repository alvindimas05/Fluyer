import { CommandRoutes } from '$lib/constants/CommandRoutes';
import { isAndroid } from '$lib/platform';
import { invoke } from '@tauri-apps/api/core';

export interface CoverArtCacheQuery {
	artist: string;
	album?: string;
	title?: string;
}

export const COVER_ART_DEBOUNCE_DELAY = isAndroid() ? 1000 : 500;

const CoverArtService = {
	getByQuery: async (query: CoverArtCacheQuery): Promise<string | null> => {
		try {
			// Returns Uint8Array (Vec<u8> from Rust)
			const bytes = await invoke<number[]>(CommandRoutes.COVER_ART_GET, {
				query
			});

			if (!bytes || bytes.length === 0) {
				return null;
			}

			// Convert to blob URL
			const arrayBuffer = new Uint8Array(bytes).buffer;
			const blob = new Blob([arrayBuffer], { type: 'image/jpeg' });
			return URL.createObjectURL(blob);
		} catch (err) {
			// Cover art not found is expected, don't log error
			return null;
		}
	}
};

export default CoverArtService;
