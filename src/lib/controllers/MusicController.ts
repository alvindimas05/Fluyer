import { invoke } from "@tauri-apps/api/core";
import {
    musicIsPlaying,
    musicCurrent,
    musicProgressIntervalId,
    musicProgressValue,
    musicList,
    musicsNext,
    musicVolume,
    musicAlbumList,
} from "$lib/stores/music";
import { get } from "svelte/store";
import type {
    MusicPlayerSync,
    MusicData,
    MusicPlayerInfo,
} from "$lib/home/music/types";
import LoadingController from "$lib/controllers/LoadingController";
import { listen } from "@tauri-apps/api/event";
import { CommandsRoute } from "$lib/commands";
import { isDesktop, isMobile } from "$lib/platform";
import {
    mobileNavigationBarHeight,
    mobileStatusBarHeight,
} from "$lib/stores/mobile";
import logHandler from "$lib/handlers/log";
import { coverArtCaches } from "$lib/stores/coverart";
import type {
    CoverArtCacheQuery,
    CoverArtResponse,
} from "$lib/handlers/coverart";

export const MusicConfig = {
    step: 0.01,
    min: 0,
    max: 10,
    vstep: 0.01,
    vmin: 0,
    vmax: 1,
    defaultTitle: "The Meaning of Life",
    defaultArtist: "Musician",
    defaultAlbumImage: "/icons/default/default-album-cover.jpg",
    defaultPlayButton: "/icons/default/play.svg",
    defaultPauseButton: "/icons/default/pause.svg",
    defaultPreviousButton: "/icons/default/previous.svg",
    defaultNextButton: "/icons/default/next.svg",
    defaultPlaylistRemoveButton: "/icons/default/remove.svg",
    defaultPlayingIcon: "/icons/default/playing.svg",
    defaultBackButton: "/icons/default/back.svg",
    defaultSpeakerButton: "/icons/default/speaker.svg",
    defaultMuteButton: "/icons/default/mute.svg",
};
const MusicController = {
    handleInitialize: () => {
        logHandler();

        MusicController.listenSyncMusic();
        MusicController.listenNextMusic();
        MusicController.setStatusBarHeight();
        MusicController.setNavigationBarHeight();
        MusicController.handleVolumeChange();
    },
    musicList: () => get(musicList),
    setMusicList: (value: MusicData[] | null) => musicList.set(value),
    getMusics: async () => {
        if (MusicController.musicList()?.length) return;
        const musics = await invoke<MusicData[] | null>(
            CommandsRoute.MUSIC_GET_ALL,
        );
        MusicController.setMusicList(musics);
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
                ? `data:image/png;base64, ${music.image}`
                : MusicConfig.defaultAlbumImage;
        }
    },

    getFullArtistFromMusic: (music: MusicData | null) => {
        if (music === null || music.artist === null)
            return MusicConfig.defaultArtist;
        return music.artist.replace(/\|\|/g, " • ");
    },

    currentMusicDuration: () =>
        MusicController.currentMusic() != null
            ? MusicController.parseProgressDuration(
                  MusicController.currentMusic()!.duration,
              )
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
            : 0,
    realProgressDuration: () => MusicController.progressDuration() * 1000,
    parseProgressDuration: (value: number) => value / 1000,
    parseProgressDurationIntoValue: (value: number, max: number) =>
        (value / max) * MusicConfig.max,
    parseProgressValueIntoDuration: (value: number, max: number) =>
        (value / MusicConfig.max) * max,

    progressDurationText: (negative = false): string => {
        let minutes = 0;
        let seconds = 0;
        for (
            seconds = negative
                ? MusicController.currentMusicDuration() -
                  MusicController.progressDuration()
                : MusicController.progressDuration();
            seconds >= 60;
            seconds -= 60
        ) {
            minutes++;
        }
        if (seconds < 0) seconds = seconds + 60;
        seconds = Math.round(seconds);
        return `${negative ? "-" : ""}${minutes}.${seconds > 9 ? seconds : "0" + seconds.toString()}`;
    },

    startProgress: ({ resetProgress } = { resetProgress: true }) => {
        const updateInterval =
            (MusicController.currentMusicDuration() / MusicConfig.max) *
            MusicConfig.step *
            1000;

        if (resetProgress) MusicController.resetProgress();
        MusicController.stopProgress();

        musicProgressIntervalId.set(
            setInterval(() => {
                MusicController.setProgressValue(
                    Math.min(
                        MusicController.progressValue() + MusicConfig.step,
                        MusicConfig.max,
                    ),
                );

                if (MusicController.progressValue() >= MusicConfig.max) {
                    MusicController.setIsPlaying(false);
                    MusicController.stopProgress();
                    MusicController.resetProgress();
                }
            }, updateInterval),
        );
    },

    nextMusic: () => {
        MusicController.sendCommandController("next");
        MusicController.stopProgress();
        MusicController.resetProgress();
    },

    listenNextMusic: () => {
        listen(CommandsRoute.MUSIC_PLAYER_NEXT, () => {
            const nextMusics = MusicController.nextMusics();
            if (nextMusics.length > 0) {
                MusicController.setCurrentMusic(nextMusics[0]);
                MusicController.startProgress();
                MusicController.play(!MusicController.isPlaying());
            } else {
                MusicController.pause();
                return;
            }
            MusicController.removeFirstNextMusics();
            MusicController.setIsPlaying(true);
        });
    },
    listenSyncMusic: () => {
        listen<MusicPlayerSync>(CommandsRoute.MUSIC_PLAYER_SYNC, async (e) => {
            const skip = e.payload.skip;
            if (skip > 0) {
                MusicController.setNextMusics(
                    MusicController.nextMusics()!.splice(0, skip),
                );
            }

            const info = e.payload.info;
            if (info.music != null) {
                MusicController.setProgressValue(
                    MusicController.parseProgressDurationIntoValue(
                        MusicController.parseProgressDuration(
                            info.currentPosition,
                        ),
                        MusicController.parseProgressDuration(
                            info.music.duration,
                        ),
                    ),
                );
            }
            MusicController.setIsPlaying(info.isPlaying);
        });
        // listen<MusicPlayerInfo>(CommandsRoute.MUSIC_GET_INFO, (e) => {
        // 	if (e.payload.music != null) {
        // 		MusicController.setProgressValue(
        // 			MusicController.parseProgressDurationIntoValue(
        // 				MusicController.parseProgressDuration(e.payload.currentPosition),
        // 				MusicController.parseProgressDuration(e.payload.music.duration),
        // 			),
        // 		);
        // 	}
        // 	MusicController.setIsPlaying(e.payload.isPlaying);
        // });
    },

    resetProgress: () => musicProgressValue.set(MusicConfig.min),

    stopProgress: () => {
        if (get(musicProgressIntervalId)) {
            clearInterval(get(musicProgressIntervalId)!);
            musicProgressIntervalId.set(null);
        }
    },

    getParsedDuration: (negative = false) => {
        if (MusicController.isCurrentMusicFinished()) return null;

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
    play: (sendCommand: boolean = true) => {
        musicIsPlaying.set(true);
        MusicController.startProgress({ resetProgress: false });
        if (sendCommand) MusicController.sendCommandController("play");
    },
    pause: (sendCommand: boolean = true) => {
        musicIsPlaying.set(false);
        MusicController.stopProgress();
        if (sendCommand) MusicController.sendCommandController("pause");
    },
    clear: async () => {
        MusicController.stopProgress();
        MusicController.resetProgress();
        MusicController.setNextMusics([]);
        await MusicController.sendCommandController("clear");
        return new Promise<void>(async (resolve, _) => {
            const unlisten = await listen(
                CommandsRoute.MUSIC_CONTROLLER,
                (e) => {
                    if (e.payload === "clear") {
                        resolve();
                        unlisten();
                    }
                },
            );
        });
    },
    sendCommandController: async (command: string) => {
        await invoke(CommandsRoute.MUSIC_CONTROLLER, { command });
    },

    sendCommandSetPosition: (position: number) => {
        invoke(CommandsRoute.MUSIC_POSITION_SET, {
            position: Math.trunc(position),
        });
    },

    addMusic: async (music: MusicData) => {
        await MusicController.addMusicList([music]);
    },
    addMusicToPlayList: async (path: string) =>
        await MusicController.addMusicListToPlayList([path]),
    addMusicListToPlayList: async (musicPaths: string[]) => {
        await invoke(CommandsRoute.MUSIC_PLAYLIST_ADD, {
            playlist: musicPaths,
        });
    },

    addMusicList: async (musicDataList: MusicData[]) => {
        let isCurrentMusicSet = false;
        if (
            MusicController.isCurrentMusicFinished() &&
            musicDataList.length > 0
        ) {
            isCurrentMusicSet = true;
            MusicController.setCurrentMusic(musicDataList[0]);
        }

        MusicController.addNextMusics(
            isCurrentMusicSet ? musicDataList.slice(1) : musicDataList,
        );

        let musicPaths: string[] = [];
        musicDataList.forEach((music) => musicPaths.push(music.path));
        MusicController.addMusicListToPlayList(musicPaths);
    },

    removeMusic: (index: number) => {
        MusicController.removeNextMusicsAt(index);
        invoke(CommandsRoute.MUSIC_PLAYLIST_REMOVE, { index: index + 1 });
    },

    nextMusics: () => get(musicsNext),
    setNextMusics: (value: MusicData[]) => musicsNext.set(value),
    addNextMusics: (musics: MusicData[]) => {
        musicsNext.set([...get(musicsNext), ...musics]);
    },

    removeNextMusicsAt: (index: number) =>
        musicsNext.set(
            MusicController.nextMusics().filter((_, i) => i !== index),
        ),
    removeFirstNextMusics: () =>
        musicsNext.set(MusicController.nextMusics().slice(1)),
    isCurrentMusicFinished: () => {
        return (
            MusicController.isProgressValueEnd() ||
            MusicController.currentMusic() === null ||
            get(musicProgressIntervalId) === null
        );
    },

    isProgressValueEnd: () =>
        MusicController.progressValue() >= MusicConfig.max ||
        MusicController.progressValue() <= MusicConfig.min,

    setStatusBarHeight: async () => {
        if (isDesktop()) return;
        const barHeight = await invoke<number>(
            CommandsRoute.GET_STATUS_BAR_HEIGHT,
        );
        mobileStatusBarHeight.set(barHeight);
    },
    setNavigationBarHeight: async () => {
        if (isDesktop()) return;
        const barHeight = await invoke<number>(
            CommandsRoute.GET_NAVIGATION_BAR_HEIGHT,
        );
        mobileNavigationBarHeight.set(barHeight);
    },

    volume: () => get(musicVolume),
    setVolume: (value: number) => {
        musicVolume.set(value);
        invoke(CommandsRoute.MUSIC_SET_VOLUME, {
            volume: MusicController.volume(),
        });
    },
    volumePercentage: () =>
        ((MusicController.volume() - MusicConfig.vmin) /
            (MusicConfig.vmax - MusicConfig.vmin)) *
        100,
    handleVolumeChange: () => {
        musicVolume.subscribe(() => {
            invoke(CommandsRoute.MUSIC_SET_VOLUME, {
                volume: MusicController.volume(),
            });
        });
    },

    getCoverArtCache: (query: CoverArtCacheQuery) => {
        let fquery = `${query.artist} ${query.album ?? query.title ?? ""}`;
        return (
            get(coverArtCaches).find((cache) => cache.name == fquery) ?? null
        );
    },
    addCoverArtCache: (value: CoverArtResponse) => {
        let caches = get(coverArtCaches);
        caches.push(value);
        coverArtCaches.set(caches);
    },
    setCoverArtCache: (query: CoverArtCacheQuery, value: CoverArtResponse) => {
        let fquery = `${query.artist} ${query.album ?? query.title ?? ""}`;
        let caches = get(coverArtCaches);
        caches[caches.findIndex((c) => c.name == fquery)] = value;
        coverArtCaches.set(caches);
    },
    withBase64: (value: string) => {
        return `data:image/png;base64,${value}`;
    },

    setMusicAlbumList: (value: MusicData[][]) => {
        musicAlbumList.set(value);
    },

    sortMusicList: (list: MusicData[]) => {
        const hasTrackNumber = list[0].trackNumber != null;
        return list.sort((a, b) => {
            if (hasTrackNumber) {
                if (
                    a.trackNumber?.includes("/") ||
                    b.trackNumber?.includes("/")
                ) {
                    a.trackNumber = a.trackNumber!.split("/")[0];
                    b.trackNumber = b.trackNumber!.split("/")[0];
                }
                return +a.trackNumber! - +b.trackNumber!;
            } else {
                return a.filename.localeCompare(b.filename);
            }
        });
    },
};

export default MusicController;
