import filterStore from '$lib/stores/filter.svelte';
import MetadataService from '$lib/services/MetadataService.svelte';
import musicStore from '$lib/stores/music.svelte';
import { type AlbumData, type MusicData, MusicListType } from '$lib/features/music/types';
import ProgressService from '$lib/services/ProgressService.svelte';
import { COVER_ART_DEBOUNCE_DELAY } from '$lib/services/CoverArtService.svelte';

export function useAlbumItem(musicList: MusicData[], index: number, getVisible: () => boolean = () => true) {
	let music = $derived(musicList[0]);

	let isValidFilterAlbum = $derived(
		filterStore.album && music.album && filterStore.album.name === music.album
	);

	let albumImage = $state<Promise<string | null> | null>(null);
	let currentBlobUrl: string | null = null;

	// Use $effect with cleanup to cancel pending requests when component unmounts
	$effect(() => {
		// Only fetch image when visible
		const isVisible = getVisible();
		if (!isVisible) return;

		let cancelled = false;
		const timeoutId = setTimeout(async () => {
			if (cancelled) return;
			const imagePromise = MetadataService.getMusicCoverArt(music);
			albumImage = imagePromise;

			// Track the blob URL for cleanup
			const url = await imagePromise;
			if (!cancelled && url) {
				// Revoke previous blob URL if exists
				if (currentBlobUrl) {
					URL.revokeObjectURL(currentBlobUrl);
				}
				currentBlobUrl = url;
			}
		}, COVER_ART_DEBOUNCE_DELAY);

		return () => {
			cancelled = true;
			clearTimeout(timeoutId);
			// Revoke blob URL on cleanup
			if (currentBlobUrl) {
				URL.revokeObjectURL(currentBlobUrl);
				currentBlobUrl = null;
			}
		};
	});

	async function setFilterAlbum() {
		const isAlbumType = musicStore.listType === MusicListType.Album;
		musicStore.listType = MusicListType.All;
		filterStore.album = {
			name: music.album,
			artist: music.albumArtist ?? music.artist,
			year: MetadataService.getYearFromDate(music.date),
			duration: ProgressService.formatDuration(
				musicList.map((m) => m.duration).reduce((a, b) => a + b, 0)
			),
			musicList
		} as AlbumData;
		if (isAlbumType) setTimeout(() => (musicStore.albumListUi.scrollIndex = index), 500);
	}

	return {
		get isValidFilterAlbum() {
			return isValidFilterAlbum;
		},
		get albumImage() {
			return albumImage;
		},
		get music() {
			return music;
		},
		setFilterAlbum
	};
}
