import type {MusicData} from "$lib/features/music/types";
import musicStore from "$lib/stores/music.svelte";
import TauriLibraryAPI from "$lib/tauri/TauriLibraryAPI";

const LibraryService = {
    initialize: async () => {
        await LibraryService.loadAlbumList();
        await LibraryService.loadMusicList();
    },
    loadMusicList: async () => {
        console.log("Getting music list...");
        musicStore.list = await TauriLibraryAPI.getMusicList();
    },
    loadAlbumList: async () => {
        $effect.root(() => {
            if(!musicStore.list) return;
            console.log("Refreshing album list...");
            const albumsMap: Record<string, MusicData[]> = {};

            for (const item of musicStore.list) {
                const album = item.album?.trim();
                if (!album) continue;

                if (!albumsMap[album]) albumsMap[album] = [];
                albumsMap[album].push(item);
            }

            musicStore.albumList = Object.keys(albumsMap)
                .sort()
                .map(key => LibraryService.sortMusicList(albumsMap[key]));
        });
    },
    sortMusicList: (list: MusicData[]) => {
        if (!list) return [];

        // Schwartzian transform to avoid re-computing values in the sort callback
        const mapped = list.map((original) => {
            const trackNumberString = original.trackNumber?.split("/")[0];
            const track = trackNumberString ? parseInt(trackNumberString, 10) : NaN;

            return {
                original,
                album: original.album || "",
                track: isNaN(track) ? Infinity : track, // Sort items without a valid track number last
                filename: original.filename,
            };
        });

        mapped.sort((a, b) => {
            if (a.album !== b.album) {
                return a.album.localeCompare(b.album);
            }

            if (a.track !== b.track) {
                return a.track - b.track;
            }

            return a.filename.localeCompare(b.filename);
        });

        return mapped.map((item) => item.original);
    },
};

export default LibraryService;