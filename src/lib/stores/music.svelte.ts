import {MusicConfig} from "$lib/constants/music";
import {type MusicData, MusicListType, RepeatMode} from "$lib/features/music/types";

const musicStore = $state({
    // Library
    list: undefined as MusicData[] | null | undefined,
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
    reset: false,
    albumListScrollIndex: -1,

    get currentMusic(): MusicData | null {
        return this.queue[this.currentIndex] ?? null;
    },
    get progressDuration(): number {
        return (this.progressValue / MusicConfig.max) * (this.currentMusic?.duration ?? 0);
    }
});

export default musicStore;