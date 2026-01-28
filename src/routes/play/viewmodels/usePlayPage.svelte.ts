import { isDesktop, isMobile } from '$lib/platform';
import musicStore from '$lib/stores/music.svelte';
import ProgressService from '$lib/services/ProgressService.svelte';
import { MusicConfig } from '$lib/constants/MusicConfig';
import MetadataService from '$lib/services/MetadataService.svelte';
import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
import PageService from '$lib/services/PageService.svelte';
import QueueService from '$lib/services/QueueService.svelte';
import LibraryService from '$lib/services/LibraryService.svelte';
import LyricService, { type MusicLyric } from '$lib/services/LyricService.svelte';

export function usePlayPage() {
	let music = $derived(musicStore.currentMusic);
	let progressPercentage = $derived.by(() => musicStore.progressPercentage);

	let progressDurationText = $state('');
	let progressDurationNegativeText = $state('');
	let updateProgressText = $state(true);

	let albumImage = $state<Promise<string | null> | null>(null);
	let currentBlobUrl: string | null = null;

	let lyrics = $state<MusicLyric[]>([]);
	let selectedLyricIndex = $state(0);

	let volumePercentage = $state(musicStore.volume);

	function handleButtonPlayPause() {
		if (musicStore.isPlaying) {
			MusicPlayerService.pause();
		} else {
			MusicPlayerService.play();
		}
	}

	function handleButtonPrevious() {
		MusicPlayerService.previous();
	}

	function handleButtonNext() {
		MusicPlayerService.next();
	}

	function handleButtonBack() {
		PageService.back();
	}

	async function handleButtonShuffle() {
		await MusicPlayerService.pause();
		await QueueService.resetAndAddList(await LibraryService.shuffleMusicList(musicStore.queue));
		await MusicPlayerService.play();
	}

	async function resetLyrics() {
		selectedLyricIndex = 0;
		lyrics = [];

		if (!musicStore.currentMusic) return;
		const resLyrics = await LyricService.get(music);
		if (resLyrics == null) {
			lyrics = [];
			return;
		}
		lyrics = resLyrics;
	}

	function resetSelectedLyricIndex() {
		if (lyrics.length < 1) return;

		const duration = musicStore.progressDuration / 1000;
		if (duration < lyrics[0].duration) {
			// Scrolled logic handled in view via effect
			return;
		}
		for (let i = 0; i < lyrics.length; i++) {
			if (duration < lyrics[i].duration) {
				selectedLyricIndex = i - 1;
				return;
			}
		}
		selectedLyricIndex = lyrics.length - 1;
	}

	function refreshProgressText() {
		if (!updateProgressText) return;
		progressDurationText = ProgressService.formatDuration(musicStore.progressDuration);
		progressDurationNegativeText =
			'-' +
			ProgressService.formatDuration(
				(musicStore.currentMusic?.duration ?? 0) - musicStore.progressDuration
			);
	}

	function handleProgressClick(percentage: number) {
		MusicPlayerService.seekByPercentage(percentage);
	}

	function handleProgressEnter() {
		updateProgressText = false;
	}

	function handleProgressMove(percentage: number) {
		updateProgressText = false;
		progressDurationText = ProgressService.formatDuration(
			(musicStore.currentMusic?.duration ?? 0) * (percentage / 100)
		);
		progressDurationNegativeText =
			'-' +
			ProgressService.formatDuration(
				(musicStore.currentMusic?.duration ?? 0) * ((100 - percentage) / 100)
			);
	}

	function handleProgressLeave() {
		updateProgressText = true;
		refreshProgressText();
	}

	function handleVolumeProgressClick(percentage: number) {
		musicStore.volume = percentage / 100;
	}

	// Effects
	$effect(() => {
		// Dependency on progress to trigger updates
		musicStore.progressPercentage;
		refreshProgressText();
		resetSelectedLyricIndex();
	});

	$effect(() => {
		// Image fetching
		musicStore.currentIndex;
		let cancelled = false;

		(async () => {
			const imagePromise = MetadataService.getMusicCoverArt(musicStore.currentMusic);
			albumImage = imagePromise;

			const url = await imagePromise;
			if (!cancelled && url && url.startsWith('blob:')) {
				if (currentBlobUrl) {
					URL.revokeObjectURL(currentBlobUrl);
				}
				currentBlobUrl = url;
			}
		})();

		resetLyrics();

		return () => {
			cancelled = true;
			if (currentBlobUrl) {
				URL.revokeObjectURL(currentBlobUrl);
				currentBlobUrl = null;
			}
		};
	});

	$effect(() => {
		volumePercentage = musicStore.volumePercentage;
	});

	return {
		get music() {
			return music;
		},
		get progressPercentage() {
			return progressPercentage;
		},
		get progressDurationText() {
			return progressDurationText;
		},
		get progressDurationNegativeText() {
			return progressDurationNegativeText;
		},
		get albumImage() {
			return albumImage;
		},
		get lyrics() {
			return lyrics;
		},
		get selectedLyricIndex() {
			return selectedLyricIndex;
		},
		get volumePercentage() {
			return volumePercentage;
		},
		set updateProgressText(val: boolean) {
			updateProgressText = val;
		},

		handleButtonPlayPause,
		handleButtonPrevious,
		handleButtonNext,
		handleButtonBack,
		handleButtonShuffle,
		handleProgressClick,
		handleProgressEnter,
		handleProgressMove,
		handleProgressLeave,
		handleVolumeProgressClick
	};
}
