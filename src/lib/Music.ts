import { invoke } from "@tauri-apps/api/core";

interface MusicPlayerInfo {
    current_position: number;
    is_paused: boolean;
}

export const MusicConfig = {
    step: 0.01,
    min: 0,
    max: 10,
    getProgressPercentage: (val: number) =>
        ((val - MusicConfig.min) / (MusicConfig.max - MusicConfig.min)) * 100,
};
export default class Music {
    private intervalId: ReturnType<typeof setInterval> | null = null;
    isPlaying = false;
    step = MusicConfig.step;
    duration = 0;
    min = MusicConfig.min;
    max = MusicConfig.max;
    value = 0;

    get progressPercentage() {
        return MusicConfig.getProgressPercentage(this.value);
    }

    get progressDuration() {
        return (this.value / this.max) * this.duration;
    }

    startProgress(callback: () => void) {
        console.log("Starting progress...");
        const updateInterval = (this.duration / this.max) * this.step * 1000;

        this.intervalId = setInterval(() => {
            this.value = Math.min(this.value + this.step, this.max);

            if (this.value >= this.max) {
                this.stopProgress();
            }

            callback();
        }, updateInterval);
    }

    resetProgress() {
        this.value = this.min;
    }

    stopProgress() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
            this.intervalId = null;
        }
    }

    getParsedDuration(negative = false): string {
        let minutes = 0;
        let seconds = negative
            ? this.duration - this.progressDuration
            : this.progressDuration;

        while (seconds > 60) {
            minutes++;
            seconds -= 60;
        }
        seconds = Math.floor(seconds);
        return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
    }

    play() {
        this.isPlaying = true;
        this.sendCommand();
    }

    pause() {
        this.isPlaying = false;
        this.sendCommand();
    }

    playOrPause() {
        this.isPlaying = !this.isPlaying;
        this.sendCommand();
    }

    sendCommand() {
        invoke("music_controller", {
            command: this.isPlaying ? "play" : "pause",
        });
    }
}
