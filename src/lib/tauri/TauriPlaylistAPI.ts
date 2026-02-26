import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';
import type { PlaylistData } from '$lib/features/music/types';

const TauriPlaylistAPI = {
    getAll: (): Promise<PlaylistData[]> => {
        return invoke(CommandRoutes.PLAYLIST_GET_ALL);
    },
    create: (playlist: PlaylistData): Promise<number> => {
        return invoke(CommandRoutes.PLAYLIST_CREATE, { playlist });
    },
    delete: (id: number): Promise<void> => {
        return invoke(CommandRoutes.PLAYLIST_DELETE, { id });
    },
    saveImage: (imageData: number[], filename: string): Promise<string> => {
        return invoke(CommandRoutes.PLAYLIST_SAVE_IMAGE, { imageData, filename });
    }
};

export default TauriPlaylistAPI;
