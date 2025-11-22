import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/commands";
import type {MusicData} from "$lib/features/music/types";

const TauriQueueAPI = {
    goTo: (index: number) => {
        return invoke(CommandRoutes.MUSIC_PLAYLIST_GOTO, { index });
    },
    add: (list: MusicData[]) => {
        return invoke(CommandRoutes.MUSIC_PLAYLIST_ADD, {
            playlist: list,
        });
    },
    moveTo: (from: number, to: number) => {
        return invoke(CommandRoutes.MUSIC_PLAYLIST_MOVETO, {
            from,
            to
        });
    },
};

export default TauriQueueAPI;