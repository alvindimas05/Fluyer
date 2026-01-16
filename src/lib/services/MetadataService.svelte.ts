import { type MusicData } from '$lib/features/music/types';
import { MusicConfig } from '$lib/constants/MusicConfig';
import musicStore from '$lib/stores/music.svelte';
import TauriMetadataAPI from '$lib/tauri/TauriMetadataAPI';

// Fast magic bytes validation - O(1) check
const isValidImageBuffer = (buffer: ArrayBuffer): boolean => {
	if (buffer.byteLength < 4) return false;
	const bytes = new Uint8Array(buffer);
	// JPEG: FF D8 FF
	if (bytes[0] === 0xff && bytes[1] === 0xd8 && bytes[2] === 0xff) return true;
	// PNG: 89 50 4E 47
	if (bytes[0] === 0x89 && bytes[1] === 0x50 && bytes[2] === 0x4e && bytes[3] === 0x47) return true;
	// BMP: 42 4D
	if (bytes[0] === 0x42 && bytes[1] === 0x4d) return true;
	return false;
};

const MetadataService = {
	getMusicCoverArt: async (music?: MusicData) => {
		if (!music) return MusicConfig.defaultAlbumImage;
		try {
			const arrayBuffer = await TauriMetadataAPI.getMusicCoverArt(music.path);
			if (arrayBuffer !== null && isValidImageBuffer(arrayBuffer)) {
				const blob = new Blob([arrayBuffer], { type: 'image/jpeg' });
				return URL.createObjectURL(blob);
			}
		} catch (e) {}
		// if (music.title == null || music.artist == null)
		return MusicConfig.defaultAlbumImage;
		// const coverArt = await CoverArtService.getByQuery({
		//     artist: music.artist!,
		//     title: music.album ? undefined : music.title!,
		//     album: music.album ?? undefined,
		// }, size);
		// return coverArt
		//     ? MetadataService.withBase64(coverArt)
		//     : MusicConfig.defaultAlbumImage;
	},
	getFolderCoverArt: async (folderPath: string) => {
		const path = await TauriMetadataAPI.getFolderCoverArtPath(folderPath);

		const music = musicStore.list?.find((m) => m.path === path);
		if (!music) return MusicConfig.defaultAlbumImage;

		return MetadataService.getMusicCoverArt(music);
	},
	withBase64: (value: string) => {
		return `data:image/png;base64,${value}`;
	},
	formatSampleRate: (sampleRate?: number) => {
		if (!sampleRate || sampleRate <= 0) return 'Unknown';

		const khz = sampleRate / 1000;
		const commonRates = [44.1, 48.0, 88.2, 96.0, 176.4, 192.0];

		// Allow slight floating point tolerance
		const isCommon = commonRates.some((r) => Math.abs(r - khz) < 0.05);

		if (isCommon) {
			return `${khz.toFixed(1)} kHz`;
		}

		return `${sampleRate} Hz`;
	},
	getYearFromDate: (date?: string) => {
		if (!date) return null;

		if (/^\d{4}-\d{2}-\d{2}$/.test(date)) {
			// Matches full date like "2025-01-01"
			return date.slice(0, 4);
		} else if (/^\d{4}$/.test(date)) {
			// Matches just a year like "2025"
			return date;
		}
		// Fallback if format is unexpected
		return date;
	}
};

export default MetadataService;
