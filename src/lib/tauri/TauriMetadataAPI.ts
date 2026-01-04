import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/constants/CommandRoutes";
import type {CoverArtSize} from "$lib/services/CoverArtService.svelte";

const TauriMetadataAPI = {
    getMusicCoverArt: (path: string, size?: CoverArtSize) => {
        return invoke<ArrayBuffer>(CommandRoutes.MUSIC_GET_IMAGE, {
            path,
            size,
        });
    },
    getFolderCoverArtPath: (path: string) => {
        return invoke<string>(
            CommandRoutes.FOLDER_GET_FIRST_MUSIC_PATH,
            { path }
        );
    }
};

export default TauriMetadataAPI;