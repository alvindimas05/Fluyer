import {type MusicData} from "$lib/features/music/types";
import {MusicConfig} from "$lib/constants/MusicConfig";
import musicStore from "$lib/stores/music.svelte";
import TauriMetadataAPI from "$lib/tauri/TauriMetadataAPI";
import CoverArtService, {CoverArtSize} from "$lib/services/CoverArtService.svelte";

const MetadataService = {
    getMusicCoverArt: async (
        music?: MusicData,
        size?: CoverArtSize,
    ) => {
        if (!music) return MusicConfig.defaultAlbumImage;
        try {
            console.log(`Fetching cover art for ${music.path}`);
            const image = await TauriMetadataAPI.getMusicCoverArt(music.path, size);
            if (image !== null) return MetadataService.withBase64(image);
        } catch (e) {}
        if (music.title == null || music.artist == null)
            return MusicConfig.defaultAlbumImage;
        const coverArt = await CoverArtService.getByQuery({
            artist: music.artist!,
            title: music.album ? undefined : music.title!,
            album: music.album ?? undefined,
        }, size);
        return coverArt
            ? MetadataService.withBase64(coverArt)
            : MusicConfig.defaultAlbumImage;
    },
    getFolderCoverArt: async (folderPath: string, size?: CoverArtSize) => {
        const path = await TauriMetadataAPI.getFolderCoverArtPath(folderPath);

        const music = musicStore.list?.find(m => m.path === path);
        if (!music) return MusicConfig.defaultAlbumImage;

        return MetadataService.getMusicCoverArt(music, size);
    },
    withBase64: (value: string) => {
        return `data:image/png;base64,${value}`;
    },
    formatSampleRate: (sampleRate?: number) => {
        if (!sampleRate || sampleRate <= 0) return "Unknown";

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
    },
};

export default MetadataService;