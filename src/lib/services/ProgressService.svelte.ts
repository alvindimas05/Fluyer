import musicStore from "$lib/stores/music.svelte";
import {MusicConfig} from "$lib/constants/music";

const ProgressService = {
    start: () => {
        const updateInterval =
            (musicStore.currentMusic?.duration ?? 0 / MusicConfig.max) *
            MusicConfig.step *
            1000;

        ProgressService.stop();

        musicStore.progressIntervalId = setInterval(() => {
            musicStore.progressValue =
                Math.min(
                    musicStore.progressValue + MusicConfig.step,
                    MusicConfig.max,
                );

            if (musicStore.progressValue >= MusicConfig.max) {
                musicStore.isPlaying = false;
                ProgressService.stop();
                ProgressService.start();
            }
        }, updateInterval);
    },
    stop: () => {
        if(!musicStore.progressIntervalId) return;
        clearInterval(musicStore.progressIntervalId);
        musicStore.progressIntervalId = null;
    },
    reset: () => {
        musicStore.progressValue = 0;
    },
    get value() {
        return musicStore.progressValue;
    },
    set value(value: number) {
        musicStore.progressValue = (value / musicStore.currentMusic!!.duration) * MusicConfig.max;
    },
    formatDuration: (value: number, negative?: boolean) => {
        // detect ms vs sec
        const isMs = value > 10000;
        const seconds = isMs ? value / 1000 : value;

        let totalSeconds = Math.max(0, seconds);
        let minutes = Math.floor(totalSeconds / 60);
        let secs = Math.round(totalSeconds % 60);

        if (secs === 60) {
            minutes += 1;
            secs = 0;
        }

        return `${negative ? "-" : ""}${minutes}:${secs.toString().padStart(2, "0")}`;
    },

    getDurationText: (negative?: boolean) => {
        return ProgressService.formatDuration(
            negative
                ? musicStore.currentMusic?.duration ?? 0 -
                musicStore.progressDuration
                : musicStore.progressDuration,
            negative,
        );
    },
}

export default ProgressService;