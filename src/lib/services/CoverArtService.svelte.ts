import { isAndroid } from '$lib/platform';
import TauriCoverArtAPI from '$lib/tauri/TauriCoverArtAPI';

export interface CoverArtCacheQuery {
	artist: string;
	album?: string;
	title?: string;
}

export enum CoverArtSize {
	MusicItem = 72,
	AlbumItem = 300,
	PlayerBar = 72,
	QueueItem = 72
}

export const COVER_ART_DEBOUNCE_DELAY = isAndroid() ? 1000 : 500;

const CoverArtService = {
	getByQuery: async (query: CoverArtCacheQuery, size?: number): Promise<string | null> => {
		try {
			const bytes = await TauriCoverArtAPI.getCoverArt(query, size);

			if (!bytes || bytes.length === 0) {
				return null;
			}

			const arrayBuffer = new Uint8Array(bytes).buffer;
			const blob = new Blob([arrayBuffer], { type: 'image/jpeg' });
			return URL.createObjectURL(blob);
		} catch (err) {
			return null;
		}
	}
};

export default CoverArtService;
