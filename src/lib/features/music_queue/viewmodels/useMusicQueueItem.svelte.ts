import type { MusicData } from '$lib/features/music/types';
import musicStore from '$lib/stores/music.svelte';
import MetadataService from '$lib/services/MetadataService.svelte';
import QueueService from '$lib/services/QueueService.svelte';

export function useMusicQueueItem(
    getMusic: () => MusicData,
    getUuid: () => string,
    getVisible: () => boolean = () => true
) {
    let albumImage = $state<Promise<string | null> | null>(null);
    let currentBlobUrl: string | null = null;

    // Derived values need to be inside the function scope or accessed via getters if they depend on arguments
    const music = $derived(getMusic());
    const uuid = $derived(getUuid());
    const visible = $derived(getVisible());

    const index = $derived(musicStore.queueIds.indexOf(uuid));
    const isPlaying = $derived(musicStore.currentIndex === index);
    const isPrevious = $derived(index < musicStore.currentIndex);

    // Fetch album image only when visible, with blob URL cleanup
    $effect(() => {
        // Track dependencies synchronously
        const currentMusic = music;
        const isVisible = visible;

        if (!isVisible) return;

        let cancelled = false;

        (async () => {
            const imagePromise = MetadataService.getMusicCoverArt(currentMusic);
            albumImage = imagePromise;

            const url = await imagePromise;
            if (!cancelled && url && url.startsWith('blob:')) {
                if (currentBlobUrl) {
                    URL.revokeObjectURL(currentBlobUrl);
                }
                currentBlobUrl = url;
            }
        })();

        return () => {
            cancelled = true;
            if (currentBlobUrl) {
                URL.revokeObjectURL(currentBlobUrl);
                currentBlobUrl = null;
            }
        };
    });

    function removePlaylist() {
        QueueService.remove(index);
    }

    function goToPlaylist() {
        QueueService.goTo(index);
    }

    return {
        get isPlaying() { return isPlaying; },
        get isPrevious() { return isPrevious; },
        get albumImage() { return albumImage; },
        removePlaylist,
        goToPlaylist
    };
}
