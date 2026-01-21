import { MusicConfig } from '$lib/constants/MusicConfig';
import { type MusicData, MusicListType, RepeatMode } from '$lib/features/music/types';

const musicStore = $state({
	// Library
	list: undefined as MusicData[] | null | undefined,
	listIds: [] as string[],
	albumList: [] as MusicData[][],
	listType: MusicListType.All,

	// Playback
	isPlaying: false,
	currentIndex: -1,
	queue: [] as MusicData[],
	queueIds: [] as string[],
	repeatMode: RepeatMode.None,

	// Progress
	progressValue: 0,
	progressIntervalId: null as null | ReturnType<typeof setInterval>,

	// Audio
	volume: 1,

	// UI State
	albumListUi: {
		scrollIndex: -1,
		scrollLeft: -1
	},

	get currentMusic(): MusicData | undefined {
		return this.queue[this.currentIndex] ?? undefined;
	},

	get progressDuration(): number {
		return (this.progressValue / MusicConfig.max) * (this.currentMusic?.duration ?? 0);
	},
	get progressPercentage(): number {
		return ((this.progressValue - MusicConfig.min) / (MusicConfig.max - MusicConfig.min)) * 100;
	},

	get volumePercentage(): number {
		return ((musicStore.volume - MusicConfig.vmin) / (MusicConfig.vmax - MusicConfig.vmin)) * 100;
	}
});

export default musicStore;
