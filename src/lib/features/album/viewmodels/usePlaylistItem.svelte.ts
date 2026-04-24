import type { PlaylistData } from '$lib/features/music/types';
import playlistStore from '$lib/stores/playlist.svelte';
import { COVER_ART_DEBOUNCE_DELAY } from '$lib/services/CoverArtService.svelte';
import PlaylistService from '$lib/services/PlaylistService.svelte';

export function usePlaylistItem(getPlaylist: () => PlaylistData, getVisible: () => boolean) {
	let albumImage = $state<string | null>(null);
	let currentBlobUrl: string | null = null;

	$effect(() => {
		const isVisible = getVisible();
		if (!isVisible) return;

		let cancelled = false;
		const timeoutId = setTimeout(async () => {
			const id = getPlaylist().id;
			if (cancelled || id === undefined) return;
			const url = await PlaylistService.getCoverArt(id);
			if (!cancelled) {
				if (currentBlobUrl) {
					URL.revokeObjectURL(currentBlobUrl);
				}
				if (url) {
					currentBlobUrl = url;
				}
				albumImage = url;
			}
		}, COVER_ART_DEBOUNCE_DELAY);

		return () => {
			cancelled = true;
			clearTimeout(timeoutId);
			if (currentBlobUrl) {
				URL.revokeObjectURL(currentBlobUrl);
				currentBlobUrl = null;
			}
		};
	});

	function selectPlaylist() {
		playlistStore.selectedPlaylist = getPlaylist();
	}

	return {
		selectPlaylist,
		get albumImage() {
			return albumImage;
		}
	};
}
