import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';
import type { PlaylistData } from '$lib/features/music/types';

const TauriPlaylistAPI = {
    getAll: () => {
        return invoke<PlaylistData[]>(CommandRoutes.PLAYLIST_GET_ALL);
    },
    create: (playlist: PlaylistData) => {
        return invoke<number>(CommandRoutes.PLAYLIST_CREATE, { playlist });
    },
    delete: (id: number) => {
        return invoke(CommandRoutes.PLAYLIST_DELETE, { id });
    },
    uploadImage: () => {
        return invoke<string>(CommandRoutes.PLAYLIST_UPLOAD_IMAGE);
    },
    readImage: (id: number) => {
        return invoke<ArrayBuffer>(CommandRoutes.PLAYLIST_READ_IMAGE, { id });
    },
};

export default TauriPlaylistAPI;
