import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
    musicIsPlaying,
    musicCurrent,
    musicProgressIntervalId,
    musicProgressValue,
    musicList,
    musicsNext,
} from "$lib/stores/music";
import { get } from "svelte/store";
import type { MusicData } from "$lib/home/music/types";
import LoadingController from "$lib/controllers/LoadingController";

interface MusicPlayerInfo {
    current_position: number;
    is_paused: boolean;
}

export const MusicConfig = {
    step: 0.01,
    min: 0,
    max: 10,
    defaultTitle: "The Meaning of Life",
    defaultArtist: "Musician",
    defaultAlbumImage: "/icons/default/default-album-cover.jpg",
};
const MusicController = {
    musicList: () => get(musicList),
    setMusicList: (value: MusicData[]) => musicList.set(value),
    getMusics: async () => {
        if (MusicController.musicList().length > 0) return;
        MusicController.setMusicList(
            await invoke<MusicData[]>("music_get_all"),
        );
        LoadingController.setLoadingMusicList(true);
    },

    isPlaying: () => get(musicIsPlaying),
    setIsPlaying: (value: boolean) => musicIsPlaying.set(value),

    currentMusic: () => get(musicCurrent),
    setCurrentMusic: (value: MusicData | null) => musicCurrent.set(value),

    currentMusicAlbumImage: () => {
        return MusicController.getAlbumImageFromMusic(
            MusicController.currentMusic(),
        );
    },
    getAlbumImageFromMusic: (music: MusicData | null) => {
        if (music === null || music.image === null || music.image.length < 1)
            return MusicConfig.defaultAlbumImage;
        if (music.image === MusicConfig.defaultAlbumImage) return music.image;
        try {
            new URL(music.image);
            return music.image;
        } catch (e) {
            if (music.image.startsWith("data:image/png;base64,"))
                return music.image;
            return music.image
                ? `data:image/png;base64,${music.image}`
                : MusicConfig.defaultAlbumImage;
        }
    },

    getFullArtistFromMusic: (music: MusicData) => {
        return (
            music.artist +
            (music.album_artist && !music.artist.includes(music.album_artist)
                ? ` • ${music.album_artist}`
                : "")
        );
    },

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
                    MusicController.tryNextMusic();
                }
            }, updateInterval),
        );
    },

    tryNextMusic: (force = false) => {
        clearInterval(get(musicProgressIntervalId)!);
        musicProgressIntervalId.set(null);
        MusicController.resetProgress();

        const nextMusics = MusicController.nextMusics();
        if (nextMusics.length > 0) {
            if (force) MusicController.sendCommandController("next");

            MusicController.setCurrentMusic(nextMusics[0]);
            MusicController.play();
        } else {
            MusicController.pause();
            return;
        }
        if (nextMusics.length > 1) {
            MusicController.addMusicToPlayList(nextMusics[1].path);
        }
        MusicController.removeLastNextMusics();
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
        MusicController.startProgress();
        MusicController.sendCommandController("play");
    },
    pause: () => {
        musicIsPlaying.set(false);
        MusicController.stopProgress();
        MusicController.sendCommandController("pause");
    },
    sendCommandController: (command: string) => {
        invoke("music_controller", { command });
    },

    sendCommandSetPosition: (position: number) => {
        invoke("music_position_set", { position: Math.trunc(position) });
    },

    addMusic: async (music: MusicData) => {
        if (MusicController.isCurrentMusicFinished()) {
            MusicController.setCurrentMusic(music);
            await MusicController.addMusicToPlayList(music.path);
        } else {
            if (MusicController.nextMusics().length < 2) {
                await MusicController.addMusicToPlayList(music.path);
            }
            MusicController.addNextMusics([music]);
        }
    },
    addMusicToPlayList: async (musicPath: string) => {
        await invoke("music_playlist_add", { playlist: [musicPath] });
    },

    nextMusics: () => get(musicsNext),

    addNextMusics: (musics: MusicData[]) => {
        musicsNext.set([...get(musicsNext), ...musics]);
    },

    removeLastNextMusics: () =>
        musicsNext.set(MusicController.nextMusics().slice(0, -1)),
    isCurrentMusicFinished: () => {
        return (
            MusicController.progressValue() >= MusicConfig.max ||
            MusicController.currentMusic() === null ||
            get(musicProgressIntervalId) === null
        );
    },
};

export default MusicController;
