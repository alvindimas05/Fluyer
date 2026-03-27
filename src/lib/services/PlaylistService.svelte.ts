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
            if (arrayBuffer !== null && MetadataService.isValidImageBuffer(arrayBuffer)) {
                const blob = new Blob([arrayBuffer], { type: 'image/jpeg' });
                return URL.createObjectURL(blob);
            }
        } catch (e) { }
        return MusicConfig.defaultCoverArt;
    }
};

export default PlaylistService;