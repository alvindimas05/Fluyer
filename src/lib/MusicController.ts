import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
    musicIsPlaying,
    musicCurrent,
    musicProgressIntervalId,
    musicProgressValue,
    musicList,
} from "./stores/music";
import { get } from "svelte/store";
import type { MusicData } from "./home/music/types";

interface MusicPlayerInfo {
    current_position: number;
    is_paused: boolean;
}

export const MusicConfig = {
    step: 0.01,
    min: 0,
    max: 10,
};
const MusicController = {
    isPlaying: () => get(musicIsPlaying),
    setIsPlaying: (value: boolean) => musicIsPlaying.set(value),
    
    musicList: () => get(musicList),

    currentMusic: () => get(musicCurrent),
    currentMusicAlbumImage: () => {
        var image = MusicController.currentMusic()!.image;
        try {
            new URL(image);
            return image;
        } catch (e) {
            return `data:image/png;base64,${image}`;
        }
    },
    setCurrentMusic: (value: MusicData | null) => musicCurrent.set(value),

    currentMusicDuration: () =>
        MusicController.currentMusic() != null
            ? MusicController.currentMusic()!.duration / 1000
            : 0,
    progressValue: () => get(musicProgressValue),
    setProgressValue: (value: number) => musicProgressValue.set(value),

    progressPercentage: () =>
        ((get(musicProgressValue) - MusicConfig.min) /
            (MusicConfig.max - MusicConfig.min)) *
        100,
    progressDuration: () =>
        MusicController.currentMusic() != null
            ? (MusicController.progressValue() / MusicConfig.max) *
              MusicController.currentMusicDuration()
            : -1,
    realProgressDuration: () => MusicController.progressDuration() * 1000,

    startProgress: () => {
        const updateInterval =
            (MusicController.currentMusicDuration() / MusicConfig.max) *
            MusicConfig.step *
            1000;

        musicProgressIntervalId.set(
            setInterval(() => {
                MusicController.setProgressValue(
                    Math.min(
                        MusicController.progressValue() + MusicConfig.step,
                        MusicConfig.max,
                    ),
                );

                if (MusicController.progressValue() >= MusicConfig.max) {
                    MusicController.stopProgress();
                }
            }, updateInterval),
        );
    },

    resetProgress: () => musicProgressValue.set(MusicConfig.min),

    stopProgress: () => {
        if (get(musicProgressIntervalId)) {
            clearInterval(get(musicProgressIntervalId)!);
            musicProgressIntervalId.set(null);
        }
    },

    getParsedDuration: (negative = false) => {
        if (MusicController.currentMusic() == null) return null;

        let minutes = 0;
        let seconds = negative
            ? MusicController.currentMusicDuration() -
              MusicController.progressDuration()
            : MusicController.progressDuration();

        while (seconds > 60) {
            minutes++;
            seconds -= 60;
        }
        seconds = Math.floor(seconds);
        return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
    },
    play: () => {
        musicIsPlaying.set(true);
        MusicController.sendCommandController("play");
    },
    pause: () => {
        musicIsPlaying.set(false);
        MusicController.sendCommandController("pause");
    },
    sendCommandController: (command: string) => {
        invoke("music_controller", { command });
    },

    sendCommandSetPosition: (position: number) => {
        invoke("music_position_set", { position: Math.trunc(position) });
    },

    addMusic: async (
        musicPath: string,
        callback: (() => void) | null = null,
    ) => {
        await invoke("music_playlist_add", { playlist: [musicPath] });
        if (callback) {
            const unlisten = await listen("music_playlist_add", async (_) => {
                callback();
                unlisten();
            });
        }
    },
};

export default MusicController;
