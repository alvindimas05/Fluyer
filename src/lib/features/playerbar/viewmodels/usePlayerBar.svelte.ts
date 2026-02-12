import { PageRoutes } from '$lib/constants/PageRoutes';
import { MusicConfig } from '$lib/constants/MusicConfig';
import { type MusicData, RepeatMode } from '$lib/features/music/types';
import MetadataService from '$lib/services/MetadataService.svelte';
import musicStore from '$lib/stores/music.svelte';
import settingStore from '$lib/stores/setting.svelte';
import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
import ProgressService from '$lib/services/ProgressService.svelte';
import PageService from '$lib/services/PageService.svelte';
import playerBarStore from '$lib/stores/playerbar.svelte';
import QueueService from '$lib/services/QueueService.svelte';
import LibraryService from '$lib/services/LibraryService.svelte';

let element = $state<HTMLDivElement>();
let oldMusic: MusicData | undefined = $state(undefined);
let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $state<Promise<string | null> | null>(null);
let currentBlobUrl: string | null = null;
let currentMusicPath: string | null = null;

let progressPercentage = $state(0);
let volumePercentage = $state(0);

let isPlaying = $derived(musicStore.isPlaying);

const gridRight = $derived.by(() => {
	if (settingStore.ui.showRepeatButton && settingStore.ui.showShuffleButton)
		return 'grid-cols-[repeat(5,auto)]';
	if (settingStore.ui.showRepeatButton && settingStore.ui.showShuffleButton)
		return 'grid-cols-[repeat(4,auto)]';
	return 'grid-cols-[repeat(3,auto)]';
});

function handleButtonPlayPause() {
	if (musicStore.isPlaying) {
		musicStore.isPlaying = false;
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

async function handleButtonShuffle() {
	await MusicPlayerService.pause();

	await QueueService.resetAndAddList(await LibraryService.shuffleMusicList(musicStore.queue));

	await MusicPlayerService.play();
	ProgressService.start();
}

function redirectToPlay() {
	PageService.goTo(PageRoutes.PLAY);
}

function handleVolumeButton() {
	musicStore.volume = musicStore.volume > 0 ? 0 : 1;
}

function refresh() {
	let music = musicStore.currentMusic;

	if (!music) {
		title = MusicConfig.defaultTitle;
		artist = MusicConfig.defaultArtist;
		return;
	}

	if (oldMusic && oldMusic.path === music.path) return;

	oldMusic = music;
	title = music.title!;
	artist = music.artist;
}

function handleProgressClick(percentage: number) {
	MusicPlayerService.seekByPercentage(percentage);
}

function handleVolumeProgressClick(percentage: number) {
	musicStore.volume = percentage / 100;
}

function updatePlayerBarHeight() {
	if (element) {
		playerBarStore.height = element.offsetHeight;
	}
}

export function usePlayerBar() {
	// Fetch album image with blob URL cleanup
	$effect(() => {
		const newMusicPath = musicStore.currentMusic?.path;

		// If the music path hasn't changed, skip re-fetching
		if (currentMusicPath === newMusicPath) return;

		currentMusicPath = newMusicPath ?? null;
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

		return () => {
			cancelled = true;
			if (currentBlobUrl) {
				URL.revokeObjectURL(currentBlobUrl);
				currentBlobUrl = null;
			}
		};
	});

	$effect(() => {
		progressPercentage = musicStore.progressPercentage;
	});

	$effect(() => {
		volumePercentage = musicStore.volumePercentage;
	});

	$effect(() => {
		musicStore.currentIndex;
		musicStore.list;
		refresh();
	});

	return {
		get element() {
			return element;
		},
		set element(value) {
			element = value;
			updatePlayerBarHeight();
		},
		get title() {
			return title;
		},
		get artist() {
			return artist;
		},
		get albumImage() {
			return albumImage;
		},
		get isPlaying() {
			return isPlaying;
		},
		get progressPercentage() {
			return progressPercentage;
		},
		get volumePercentage() {
			return volumePercentage;
		},
		get gridRight() {
			return gridRight;
		},

		handleButtonPlayPause,
		handleButtonPrevious,
		handleButtonNext,
		handleButtonShuffle,
		redirectToPlay,
		handleVolumeButton,
		handleProgressClick,
		handleVolumeProgressClick,
		updatePlayerBarHeight
	};
}
