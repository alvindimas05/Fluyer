import { Modal } from "$lib/constants/Modal";
import { MusicConfig } from "$lib/constants/MusicConfig";
import type { PlaylistData } from "$lib/features/music/types";
import playlistStore from "$lib/stores/playlist.svelte";
import TauriPlaylistAPI from "$lib/tauri/TauriPlaylistAPI";
import MetadataService from "./MetadataService.svelte";
import ModalService from "./ModalService.svelte";

const PlaylistService = {
    initialize: () => {
        PlaylistService.loadPlaylist();
    },
    loadPlaylist: async () => {
        playlistStore.list = await TauriPlaylistAPI.getAll();
    },
    showModal: () => {
        ModalService.open(Modal.CreatePlaylist);
    },
    requestUploadImage: async () => {
        const uploadedImagePath = await TauriPlaylistAPI.uploadImage();
        return uploadedImagePath;
    },
    requestCreate: () => {
        playlistStore.isCreating = true;
        playlistStore.selectedPaths = [];
        playlistStore.selectedPlaylist = null;
    },
    confirmCreate: () => {
        if (playlistStore.selectedPaths.length === 0) {
            playlistStore.isCreating = false;
            return;
        }
        PlaylistService.showModal();
    },
    cancelCreation: () => {
        ModalService.close();
        playlistStore.isCreating = false;
        playlistStore.selectedPaths = [];
    },
    create: async (playlist: PlaylistData) => {
        await TauriPlaylistAPI.create(playlist);
    },
    getCoverArt: async (id: number) => {
        try {
            const arrayBuffer = await TauriPlaylistAPI.readImage(id);
            console.log(`[PlaylistService] readImage returned buffer of size: ${arrayBuffer?.byteLength}`);
            if (arrayBuffer !== null && arrayBuffer.byteLength >= 4) {
                const bytes = new Uint8Array(arrayBuffer.slice(0, 4));
                console.log(`[PlaylistService] Magic bytes: ${Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join(' ')}`);
            }
            if (arrayBuffer !== null && MetadataService.isValidImageBuffer(arrayBuffer)) {
                const blob = new Blob([arrayBuffer], { type: 'image/png' });
                return URL.createObjectURL(blob);
            } else if (arrayBuffer !== null) {
                console.warn(`[PlaylistService] Invalid image buffer for playlist ${id}`);
            }
        } catch (e) {
            console.error(e);
        }
        return MusicConfig.defaultCoverArt;
    },
    delete: async (id: number) => {
        await TauriPlaylistAPI.delete(id);
        PlaylistService.loadPlaylist();
    }
};

export default PlaylistService;