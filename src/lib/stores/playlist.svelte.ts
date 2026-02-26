import type { PlaylistData } from '$lib/features/music/types';

const playlistStore = $state({
    list: [] as PlaylistData[],
    selectedPlaylist: null as PlaylistData | null,
    isCreating: false,
    selectedPaths: [] as string[],
    showCreateModal: false
});

export default playlistStore;
