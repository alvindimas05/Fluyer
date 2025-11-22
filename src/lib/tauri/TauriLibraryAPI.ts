import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/commands";
import type {MusicData} from "$lib/features/music/types";

const TauriLibraryAPI = {
    getMusicList: async () => {
        return invoke<MusicData[] | null>(
            CommandRoutes.MUSIC_GET_ALL,
        );
    },
};

export default TauriLibraryAPI;