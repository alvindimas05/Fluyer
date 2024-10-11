import { invoke } from "@tauri-apps/api/core";

export default class Music {
    private intervalId: ReturnType<typeof setInterval> | null = null;
    isPlaying = false;
    step = .01;
    duration = 248;
    min = 0;
    max = 10;
    value = 0;

    get progressPercentage() {
        return ((this.value - this.min) / (this.max - this.min)) * 100;
    }

    get progressDuration() {
        return (this.value / this.max) * this.duration;
    }
    
    startProgress(callback: () => void) {
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
        let seconds = negative ? this.duration - this.progressDuration : this.progressDuration;

        while (seconds > 60) {
            minutes++;
            seconds -= 60;
        }
        seconds = Math.floor(seconds);
        return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
    }
    
    playOrPause() {
        this.isPlaying = !this.isPlaying;
        invoke('music_controller', { command: this.isPlaying ? "play" : "pause" });
    }
}
