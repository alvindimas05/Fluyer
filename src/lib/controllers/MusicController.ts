import { invoke } from "@tauri-apps/api/core";
import {
	musicIsPlaying,
	musicProgressValue,
	musicList,
	musicVolume,
	musicAlbumList,
	musicPlaylist,
	musicCurrentIndex,
	musicRepeatMode,
	musicReset,
} from "$lib/stores/music";
import { get } from "svelte/store";
import {
	type MusicPlayerSync,
	type MusicData,
	RepeatMode,
	MusicSize,
} from "$lib/home/music/types";
import LoadingController from "$lib/controllers/LoadingController";
import { listen } from "@tauri-apps/api/event";
import { CommandRoutes } from "$lib/commands";
import { coverArtCaches } from "$lib/stores/coverart";
import CoverArt, {
	type CoverArtCacheQuery,
	type CoverArtResponse,
} from "$lib/handlers/coverart";
import {isAndroid, isDesktop, isMobile} from "$lib/platform";
import { equalizerValues } from "$lib/stores/equalizer";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import UtilsController from "$lib/controllers/UtilsController";
import { settingBitPerfectMode } from "$lib/stores/setting";
import {playlistMoveQueue} from "$lib/home/playlist/PlaylistMoveQueue";
import {Music} from "@lucide/svelte";
import sleep from "sleep-promise";

export const MusicConfig = {
	step: 0.01,
	min: 0,
	max: 10,
	vstep: 0.01,
	vmin: 0,
	vmax: 1,
	separator: "•",
	separatorAlbum: "-",
	separatorAudio: "/",
	artistSeparator: "||",
	defaultTitle: import.meta.env.VITE_DEFAULT_MUSIC_TITLE,
	defaultArtist: import.meta.env.VITE_DEFAULT_MUSIC_ARTIST,
	defaultAlbumImage: "/icons/default/default-album-cover.png",
};
const MusicController = {
	initialize: async () => {
		MusicController.listenSyncMusic();
		MusicController.handleVolumeChange();

		MusicController.onBitPerfectModeChange(get(settingBitPerfectMode));
        // MusicController.listenProgressAndroid();
		settingBitPerfectMode.subscribe(MusicController.onBitPerfectModeChange);
	},
	musicList: () => get(musicList),
	setMusicList: (value: MusicData[] | null) => musicList.set(value),
	getMusics: async (force = false) => {
		if (MusicController.musicList()?.length && !force) return;
		const musics = await invoke<MusicData[] | null>(
			CommandRoutes.MUSIC_GET_ALL,
		);
		MusicController.setMusicList(musics);
		LoadingController.setLoadingMusicList(true);
	},

	get isPlaying() {
		return get(musicIsPlaying);
	},
	setIsPlaying: (value: boolean) => musicIsPlaying.set(value),

	get currentMusic() {
		return get(musicPlaylist)[get(musicCurrentIndex)] ?? null;
	},
	get currentMusicIndex() {
		return get(musicCurrentIndex);
	},
	setCurrentMusicIndex: (value: number) => musicCurrentIndex.set(value),
	getMusicByIndex: (index: number) => MusicController.musicPlaylist[index],

	get musicPlaylist() {
		return get(musicPlaylist);
	},
	addMusicPlaylist: (value: MusicData[]) =>
		musicPlaylist.set([...MusicController.musicPlaylist, ...value]),
	removeMusicPlaylist: (index: number) => {
		let playlist = MusicController.musicPlaylist;
		playlist.splice(index, 1);
		musicPlaylist.set(playlist);
	},

	get currentMusicAlbumImage() {
		return MusicController.getAlbumImageFromMusic(MusicController.currentMusic);
	},
	getAlbumImageFromMusic: async (
		music: MusicData | null,
		size: MusicSize | null = null,
	) => {
		if (music === null) return MusicConfig.defaultAlbumImage;
		const imageSize = size ? size.toString() : null;
		try {
			const image = await invoke<string>(CommandRoutes.MUSIC_GET_IMAGE, {
				path: music?.path,
				size: imageSize,
			});
			if (image !== null) return UtilsController.withBase64(image);
		} catch (e) {}
		if (music.title == null || music.artist == null)
			return MusicConfig.defaultAlbumImage;
		const coverArt = await CoverArt.getImageFromQuery(
			{
				artist: music.artist!,
				title: music.album ? undefined : music.title!,
				album: music.album ?? undefined,
			},
			imageSize,
		);
		return coverArt
			? UtilsController.withBase64(coverArt)
			: MusicConfig.defaultAlbumImage;
	},

	getFullArtistFromMusic: (music: MusicData | null) => {
		if (music === null || music.artist === null)
			return MusicConfig.defaultArtist;
		return music.artist.replace(/\|\|/g, ` ${MusicConfig.separator} `);
	},

	get currentMusicDuration() {
		return MusicController.currentMusic != null
			? MusicController.parseProgressDuration(
					MusicController.currentMusic!.duration,
				)
			: 0;
	},
	get currentMusicRealDuration() {
		return MusicController.currentMusic != null
			? MusicController.currentMusicDuration * 1000
			: 0;
	},
	get progressValue() {
		return get(musicProgressValue);
	},
	setProgressValue: (value: number) => musicProgressValue.set(value),

	get progressPercentage() {
		return (
			((get(musicProgressValue) - MusicConfig.min) /
				(MusicConfig.max - MusicConfig.min)) *
			100
		);
	},
	get progressDuration() {
		return MusicController.currentMusic != null
			? (MusicController.progressValue / MusicConfig.max) *
					MusicController.currentMusicDuration
			: 0;
	},
	get realProgressDuration() {
		return MusicController.progressDuration * 1000;
	},
	parseProgressDuration: (value: number) => value / 1000,
	parseProgressDurationIntoValue: (value: number, max: number) =>
		(value / max) * MusicConfig.max,
	parseProgressValueIntoDuration: (value: number, max: number) =>
		(value / MusicConfig.max) * max,

	parseProgressPercentageIntoValue: (value: number) =>
		(value / 100) * MusicConfig.max,

    parseProgressDurationIntoText: (value: number, negative = false) => {
        let totalSeconds = Math.max(0, value);
        let minutes = Math.floor(totalSeconds / 60);
        let seconds = Math.round(totalSeconds % 60);

        if (seconds === 60) {
            minutes += 1;
            seconds = 0;
        }

        return `${negative ? "-" : ""}${minutes}:${seconds.toString().padStart(2, "0")}`;
    },

	mpvMusicCurrentDuration: () =>
		invoke<number | null>(CommandRoutes.MUSIC_GET_CURRENT_DURATION),
	progressDurationText: (negative = false): string => {
		return MusicController.parseProgressDurationIntoText(
			negative
				? MusicController.currentMusicDuration -
						MusicController.progressDuration
				: MusicController.progressDuration,
			negative,
		);
	},
	parsePercentageProgressDurationIntoText: (
		value: number,
		negative = false,
	) => {
		return MusicController.parseProgressDurationIntoText(
			negative
				? MusicController.currentMusicDuration * ((100 - value) / 100)
				: MusicController.currentMusicDuration * (value / 100),
			negative,
		);
	},
	getYearFromDate: (date: string | null) => {
		if (!date) return null;

		if (/^\d{4}-\d{2}-\d{2}$/.test(date)) {
			// Matches full date like "2025-01-01"
			return date.slice(0, 4);
		} else if (/^\d{4}$/.test(date)) {
			// Matches just a year like "2025"
			return date;
		}
		// Fallback if format is unexpected
		return date;
	},

	previousMusic: () => {
		if (MusicController.currentMusicIndex <= 0) return;
		MusicController.gotoPlaylist(MusicController.currentMusicIndex - 1);
	},

	nextMusic: async () => {
		if (
			MusicController.musicPlaylist.length === 0 ||
			MusicController.currentMusicIndex >=
				MusicController.musicPlaylist.length - 1
		)
			return;
		MusicController.sendCommandController("next");
	},

    listenSyncMusic: () => {
        listen<MusicPlayerSync>(CommandRoutes.MUSIC_PLAYER_SYNC, async (e) => {
            const index = e.payload.index;
            MusicController.setIsPlaying(e.payload.isPlaying);

            let position = e.payload.currentPosition ? e.payload.currentPosition / 1000 : null;

            // Handle index changes first - update immediately when index changes
            // regardless of position value
            if(index > -1 && index !== MusicController.currentMusicIndex && position && position >= 0) {
                console.log("Setting index: ", index)
                console.log("Current position: ", position);
                MusicController.setCurrentMusicIndex(index);
            }

            // Note: For some reason, libmpv will give negative position when it's almost 2-3 seconds finished.
            // So to get that missing position, just substract it with current music duration.
            if(position && position < 0 && MusicController.currentMusicDuration > 0 && index > 0){
                position = MusicController.currentMusicDuration + position;
                console.log("Setting negative position: ", position);
            }

            // Only update progress for valid positive positions within the track duration
            if(position && position >= 0 && position <= MusicController.currentMusicDuration && index > -1){
                console.log("Setting position: ", position);
                MusicController.setProgressValue(
                    MusicController.parseProgressDurationIntoValue(position, MusicController.currentMusicDuration)
                );
            }
        });
    },
    requestSync: () => {
        return invoke(CommandRoutes.MUSIC_PLAYER_REQUEST_SYNC);
    },
    listenProgressAndroid: async () => {
        if(!isAndroid()) return;

        while(true){
            await sleep(500);
            if(!MusicController.isPlaying) return;
            await MusicController.requestSync();
        }
    },

	resetProgress: () => musicProgressValue.set(MusicConfig.min),

	getParsedDuration: (negative = false) => {
		if (MusicController.isCurrentMusicFinished) return null;

		let seconds = negative
			? MusicController.currentMusicDuration - MusicController.progressDuration
			: MusicController.progressDuration;

		return MusicController.parseSecondsIntoText(seconds);
	},
	parseSecondsIntoText: (seconds: number) => {
		return MusicController.parseMilisecondsIntoText(seconds * 1000);
	},
	parseMilisecondsIntoText: (milliseconds: number) => {
		const totalSeconds = Math.floor(milliseconds / 1000);
		const hours = Math.floor(totalSeconds / 3600);
		const minutes = Math.floor((totalSeconds % 3600) / 60);
		const seconds = totalSeconds % 60;

		const paddedSeconds = `${seconds < 10 ? "0" : ""}${seconds}`;

		if (hours > 0) {
			const paddedMinutes = `${minutes < 10 ? "0" : ""}${minutes}`;
			return `${hours}:${paddedMinutes}:${paddedSeconds}`;
		}

		return `${minutes}:${paddedSeconds}`;
	},

	parseSampleRateIntoText: (sampleRate: number | null) => {
		if (!sampleRate || sampleRate <= 0) return "Unknown";

		const khz = sampleRate / 1000;
		const commonRates = [44.1, 48.0, 88.2, 96.0, 176.4, 192.0];

		// Allow slight floating point tolerance
		const isCommon = commonRates.some((r) => Math.abs(r - khz) < 0.05);

		if (isCommon) {
			return `${khz.toFixed(1)} kHz`;
		}

		return `${sampleRate} Hz`;
	},

	play: async (sendCommand: boolean = true) => {
		if (MusicController.musicPlaylist.length === 0) return;
		if (
			MusicController.isCurrentMusicFinished &&
			MusicController.isProgressValueEnd &&
			MusicController.currentMusicIndex < MusicController.musicPlaylist.length
		) {
			MusicController.gotoPlaylist(MusicController.currentMusicIndex);
		}

		musicIsPlaying.set(true);
		if (sendCommand) {
			await MusicController.sendCommandController("play");
		}
	},
	pause: (sendCommand: boolean = true) => {
		musicIsPlaying.set(false);
		if (sendCommand) MusicController.sendCommandController("pause");
	},
	sendCommandController: async (command: string) => {
		await invoke(CommandRoutes.MUSIC_CONTROLLER, { command });
	},

	sendCommandSetPosition: (position: number) => {
		invoke(CommandRoutes.MUSIC_POSITION_SET, {
			position: Math.trunc(position),
		});
	},

	addMusic: async (music: MusicData) => {
		await MusicController.addMusicList([music]);
	},
	removeMusic: (index: number) => {
		MusicController.removeMusicPlaylist(index);
		if (index <= MusicController.currentMusicIndex)
			MusicController.setCurrentMusicIndex(
				MusicController.currentMusicIndex - 1,
			);
		invoke(CommandRoutes.MUSIC_PLAYLIST_REMOVE, { index });
	},

	addSinkMusic: async (musicData: MusicData) =>
		await MusicController.addSinkMusics([musicData]),
	addSinkMusics: async (musicDataList: MusicData[]) => {
		await invoke(CommandRoutes.MUSIC_PLAYLIST_ADD, {
			playlist: musicDataList,
		});
	},

	addMusicList: async (
		musicDataList: MusicData[],
		options?: {
			forceSetCurrentMusicIndex?: boolean;
			resetPlaylist?: boolean;
		},
	) => {
		let isPlaylistEmpty = !MusicController.musicPlaylist.length;
		await MusicController.addSinkMusics(
			musicDataList.map((music) => {
				const { path, title, artist } = music;
				if (isDesktop()) return { path } as MusicData;
				return { path, title, artist } as MusicData;
			}),
		);

		if (options?.resetPlaylist) musicPlaylist.set(musicDataList);
		else MusicController.addMusicPlaylist(musicDataList);

		// if (
		// 	options?.forceSetCurrentMusicIndex ||
		// 	(MusicController.currentMusicIndex === -1 && isPlaylistEmpty)
		// )
		// 	MusicController.setCurrentMusicIndex(0);
	},

	get isCurrentMusicFinished() {
		return (
			MusicController.isProgressValueEnd ||
			MusicController.currentMusic === null
		);
	},

	get isProgressValueEnd() {
		return (
			MusicController.progressValue >= MusicConfig.max ||
			MusicController.progressValue <= MusicConfig.min
		);
	},

	get volume() {
		return get(musicVolume);
	},
	setVolume: (value: number) => {
		musicVolume.set(value);
	},
	get volumePercentage() {
		return (
			((MusicController.volume - MusicConfig.vmin) /
				(MusicConfig.vmax - MusicConfig.vmin)) *
			100
		);
	},
	handleVolumeChange: () => {
		musicVolume.subscribe(async (value) => {
			await PersistentStoreController.volume.set(value);
			await MusicController.applyVolume(value);
		});
	},
	applyVolume: (volume: number) => {
		return invoke(CommandRoutes.MUSIC_SET_VOLUME, { volume });
	},

	getCoverArtCache: (query: CoverArtCacheQuery) => {
		let fquery = `${query.artist} ${query.album ?? query.title ?? ""}`;
		return get(coverArtCaches).find((cache) => cache.name == fquery) ?? null;
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

	setMusicAlbumList: (value: MusicData[][]) => {
		musicAlbumList.set(value);
	},

	sortMusicList: (list: MusicData[]) => {
		if (!list) return [];

		// Schwartzian transform to avoid re-computing values in the sort callback
		const mapped = list.map((original) => {
			const trackNumberString = original.trackNumber?.split("/")[0];
			const track = trackNumberString ? parseInt(trackNumberString, 10) : NaN;

			return {
				original,
				album: original.album || "",
				track: isNaN(track) ? Infinity : track, // Sort items without a valid track number last
				filename: original.filename,
			};
		});

		mapped.sort((a, b) => {
			if (a.album !== b.album) {
				return a.album.localeCompare(b.album);
			}

			if (a.track !== b.track) {
				return a.track - b.track;
			}

			return a.filename.localeCompare(b.filename);
		});

		return mapped.map((item) => item.original);
	},

	gotoPlaylist: (index: number) => {
		invoke(CommandRoutes.MUSIC_PLAYLIST_GOTO, { index });
	},

	reset: async () => {
		MusicController.pause();
		await MusicController.sendCommandController("clear");
		musicCurrentIndex.set(-1);
		musicPlaylist.set([]);
		MusicController.resetProgress();
	},

	resetAndAddMusic: async (musicData: MusicData) => {
		await MusicController.resetAndAddMusicList([musicData]);
	},

	resetAndAddMusicList: async (musicList: MusicData[]) => {
		MusicController.pause();
		await MusicController.sendCommandController("clear");
		MusicController.resetProgress();

		// if (MusicController.currentMusicIndex >= 0)
		// 	MusicController.setCurrentMusicIndex(0);
		musicReset.set(true);
		await MusicController.addMusicList(musicList, {
			resetPlaylist: true,
		});
		musicReset.set(false);
	},

    seekByPercentage: (percentage: number) => {
        const clamped = Math.min(100, Math.max(0, percentage));
        const position = MusicController.currentMusicRealDuration * (clamped / 100);

        MusicController.sendCommandSetPosition(position);
    },


	toggleRepeatMode: () => {
		const nextRepeatMode = (() => {
			const currentMode = get(musicRepeatMode);
			switch (currentMode) {
				case RepeatMode.None:
					return RepeatMode.All;
				case RepeatMode.All:
					return RepeatMode.One;
				case RepeatMode.One:
					return RepeatMode.None;
			}
		})();
		musicRepeatMode.set(nextRepeatMode);
		invoke(CommandRoutes.MUSIC_CONTROLLER, {
			command: nextRepeatMode.toString(),
		});
	},
	playShuffle: async (playlist: MusicData[] | null = null) => {
		if (!playlist) playlist = MusicController.musicPlaylist;
		if (playlist.length < 2) return;

		for (let i = playlist.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[playlist[i], playlist[j]] = [playlist[j], playlist[i]];
		}
		await MusicController.resetAndAddMusicList(playlist);
		MusicController.play();
	},

    playlistMoveto: async (fromIndex: number, toIndex: number) => {
        return playlistMoveQueue.add(async () => {
            if (fromIndex === toIndex) return;

            const currentMusic = MusicController.currentMusic;
            const playlist = MusicController.musicPlaylist;
            const music = playlist[fromIndex];
            playlist.splice(fromIndex, 1);
            playlist.splice(toIndex, 0, music);

            await invoke(CommandRoutes.MUSIC_PLAYLIST_MOVETO, {
                from: fromIndex,
                to: isDesktop() && fromIndex < MusicController.currentMusicIndex
                    ? toIndex + 1
                    : toIndex,
            });

            musicPlaylist.set(playlist);
            musicCurrentIndex.set(
                playlist.findIndex((m) => m.path === currentMusic.path),
            );
        });
    },

	setEqualizer: async (values: number[]) => {
		if (isDesktop())
			await invoke(CommandRoutes.MUSIC_EQUALIZER, {
				values,
			});
	},
	applyEqualizer: async () => {
		await MusicController.setEqualizer(get(equalizerValues));
	},
	resetEqualizer: async () => {
		const values = Array(18).fill(0);
		equalizerValues.set(values);
		await PersistentStoreController.equalizer.set(values);

		await invoke(CommandRoutes.MUSIC_EQUALIZER_RESET);
	},

	getVisualizerBuffer: async (path: string) => {
		return await invoke<ArrayBuffer | null>(
			CommandRoutes.MUSIC_GET_VISUALIZER_BUFFER,
			{
				path,
			},
		);
	},

	onBitPerfectModeChange: (bitPerfectMode: boolean) => {
		MusicController.setVolume(1);
		MusicController.applyVolume(1);
		if (bitPerfectMode) {
			MusicController.applyEqualizer();
		} else {
			MusicController.resetEqualizer();
		}
		MusicController.toggleBitPerfectMode(bitPerfectMode);
	},
	toggleBitPerfectMode: (enable: boolean) => {
		return invoke(CommandRoutes.MUSIC_TOGGLE_BIT_PERFECT, { enable });
	},
};

export default MusicController;
