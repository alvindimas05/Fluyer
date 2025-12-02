import musicStore from "$lib/stores/music.svelte";
import {MusicConfig} from "$lib/constants/MusicConfig";

const ProgressService = {
    initialize: () => {

    },
    start: () => {
        ProgressService.stop();

        setTimeout(() => {
            console.log(`Starting progress with duration: ${musicStore.currentMusic?.duration}`);

            const updateInterval =
                (musicStore.currentMusic!!.duration / MusicConfig.max) *
                MusicConfig.step;

            musicStore.progressIntervalId = setInterval(() => {
                musicStore.progressValue += MusicConfig.step;

                if (musicStore.progressValue >= MusicConfig.max) {
                    console.log('Progress value ended. Stopping...');
                    musicStore.isPlaying = false;
                    ProgressService.stop();
                }
            }, updateInterval);
        }, 10);
    },
    stop: () => {
        if(!musicStore.progressIntervalId) return;
        clearInterval(musicStore.progressIntervalId);
        musicStore.progressIntervalId = null;
    },
    reset: () => {
        musicStore.progressValue = 0;
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