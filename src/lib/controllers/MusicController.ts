import { invoke } from "@tauri-apps/api/core";
import {
	musicIsPlaying,
	musicProgressIntervalId,
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
	RepeatMode, MusicSize,
} from "$lib/home/music/types";
import LoadingController from "$lib/controllers/LoadingController";
import { listen } from "@tauri-apps/api/event";
import { CommandRoutes } from "$lib/commands";
import { coverArtCaches } from "$lib/stores/coverart";
import CoverArt, {
	type CoverArtCacheQuery,
	type CoverArtResponse,
} from "$lib/handlers/coverart";
import { isDesktop, isMobile } from "$lib/platform";
import { equalizerValues } from "$lib/stores/equalizer";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import UtilsController from "$lib/controllers/UtilsController";

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

	isPlaying: () => get(musicIsPlaying),
	setIsPlaying: (value: boolean) => musicIsPlaying.set(value),

	currentMusic: () => get(musicPlaylist)[get(musicCurrentIndex)] ?? null,
	currentMusicIndex: () => get(musicCurrentIndex),
	setCurrentMusicIndex: (value: number) => musicCurrentIndex.set(value),
	getMusicByIndex: (index: number) => MusicController.musicPlaylist()[index],

	musicPlaylist: () => get(musicPlaylist),
	addMusicPlaylist: (value: MusicData[]) =>
		musicPlaylist.set([...MusicController.musicPlaylist(), ...value]),
	removeMusicPlaylist: (index: number) => {
		let playlist = MusicController.musicPlaylist();
		playlist.splice(index, 1);
		musicPlaylist.set(playlist);
	},

	currentMusicAlbumImage: () => {
		return MusicController.getAlbumImageFromMusic(
			MusicController.currentMusic(),
		);
	},
	getAlbumImageFromMusic: async (music: MusicData | null, size: MusicSize | null = null) => {
		if (music === null) return MusicConfig.defaultAlbumImage;
		const imageSize = size ? size.toString() : null;
		try {
			const image = await invoke<string>(CommandRoutes.MUSIC_GET_IMAGE, {
				path: music?.path, size: imageSize,
			});
			if (image !== null) return UtilsController.withBase64(image);
		} catch (e) {}
		if (music.title == null || music.artist == null)
			return MusicConfig.defaultAlbumImage;
		const coverArt = await CoverArt.getImageFromQuery({
			artist: music.artist!,
			title: music.album ? undefined : music.title!,
			album: music.album ?? undefined,
		}, imageSize);
		return coverArt
			? UtilsController.withBase64(coverArt)
			: MusicConfig.defaultAlbumImage;
	},

	getFullArtistFromMusic: (music: MusicData | null) => {
		if (music === null || music.artist === null)
			return MusicConfig.defaultArtist;
		return music.artist.replace(/\|\|/g, ` ${MusicConfig.separator} `);
	},

	currentMusicDuration: () =>
		MusicController.currentMusic() != null
			? MusicController.parseProgressDuration(
					MusicController.currentMusic()!.duration,
				)
			: 0,
	currentMusicRealDuration: () =>
		MusicController.currentMusic() != null
			? MusicController.currentMusicDuration() * 1000
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

	parseProgressPercentageIntoValue: (value: number) =>
		(value / 100) * MusicConfig.max,

	parseProgressDurationIntoText: (value: number, negative = false) => {
		let minutes = 0;
		let seconds: number;
		for (seconds = value; seconds >= 60; seconds -= 60) {
			minutes++;
		}
		if (seconds < 0) seconds = seconds + 60;
		seconds = Math.round(seconds);
		return `${negative ? "-" : ""}${minutes}:${seconds > 9 ? seconds : "0" + seconds.toString()}`;
	},
	mpvMusicCurrentDuration: () =>
		invoke<number | null>(CommandRoutes.MUSIC_GET_CURRENT_DURATION),
	progressDurationText: (negative = false): string => {
		return MusicController.parseProgressDurationIntoText(
			negative
				? MusicController.currentMusicDuration() -
						MusicController.progressDuration()
				: MusicController.progressDuration(),
			negative,
		);
	},
	parsePercentageProgressDurationIntoText: (
		value: number,
		negative = false,
	) => {
		return MusicController.parseProgressDurationIntoText(
			negative
				? MusicController.currentMusicDuration() * ((100 - value) / 100)
				: MusicController.currentMusicDuration() * (value / 100),
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

	previousMusic: () => {
		if (MusicController.currentMusicIndex() <= 0) return;
		MusicController.gotoPlaylist(MusicController.currentMusicIndex() - 1);
	},

	nextMusic: async () => {
		if (
			MusicController.musicPlaylist().length === 0 ||
			MusicController.currentMusicIndex() >=
				MusicController.musicPlaylist().length - 1
		)
			return;
		MusicController.sendCommandController("next");
	},

	listenSyncMusic: () => {
		listen<MusicPlayerSync>(CommandRoutes.MUSIC_PLAYER_SYNC, async (e) => {
			MusicController.setCurrentMusicIndex(e.payload.index);
			if (e.payload.isPlaying)
				MusicController.startProgress({ resetProgress: true });
			else MusicController.stopProgress();

			MusicController.setProgressValue(
				MusicController.parseProgressDurationIntoValue(
					MusicController.parseProgressDuration(e.payload.currentPosition),
					MusicController.parseProgressDuration(
						MusicController.musicPlaylist()[e.payload.index].duration,
					),
				),
			);
			MusicController.setIsPlaying(e.payload.isPlaying);
		});
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

		let seconds = negative
			? MusicController.currentMusicDuration() -
				MusicController.progressDuration()
			: MusicController.progressDuration();

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
		if (MusicController.musicPlaylist().length === 0) return;
		if (
			MusicController.isCurrentMusicFinished() &&
			MusicController.isProgressValueEnd() &&
			MusicController.currentMusicIndex() <
				MusicController.musicPlaylist().length
		) {
			MusicController.gotoPlaylist(MusicController.currentMusicIndex());
		}

		musicIsPlaying.set(true);
		MusicController.startProgress({ resetProgress: false });
		if (sendCommand) {
			await MusicController.applyEqualizer();
			MusicController.sendCommandController("play");
		}
	},
	pause: (sendCommand: boolean = true) => {
		musicIsPlaying.set(false);
		MusicController.stopProgress();
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
		if (index <= MusicController.currentMusicIndex())
			MusicController.setCurrentMusicIndex(
				MusicController.currentMusicIndex() - 1,
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
		let isPlaylistEmpty = !MusicController.musicPlaylist().length;
		await MusicController.addSinkMusics(
			musicDataList.map((music) => {
				const { path, title, artist } = music;
				if (isDesktop()) return { path } as MusicData;
				return { path, title, artist } as MusicData;
			}),
		);

		if (options?.resetPlaylist) musicPlaylist.set(musicDataList);
		else MusicController.addMusicPlaylist(musicDataList);

		if (
			options?.forceSetCurrentMusicIndex ||
			(MusicController.currentMusicIndex() === -1 && isPlaylistEmpty)
		)
			MusicController.setCurrentMusicIndex(0);
	},

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

	volume: () => get(musicVolume),
	setVolume: (value: number) => {
		musicVolume.set(value);
		invoke(CommandRoutes.MUSIC_SET_VOLUME, {
			volume: MusicController.volume(),
		});
	},
	volumePercentage: () =>
		((MusicController.volume() - MusicConfig.vmin) /
			(MusicConfig.vmax - MusicConfig.vmin)) *
		100,
	handleVolumeChange: () => {
		musicVolume.subscribe(() => {
			invoke(CommandRoutes.MUSIC_SET_VOLUME, {
				volume: MusicController.volume(),
			});
		});
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
			const trackNumberString = original.trackNumber?.split('/')[0];
			const track = trackNumberString ? parseInt(trackNumberString, 10) : NaN;

			return {
				original,
				album: original.album || '',
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
		MusicController.stopProgress();
		MusicController.resetProgress();
	},

	resetAndAddMusic: async (musicData: MusicData) => {
		await MusicController.resetAndAddMusicList([musicData]);
	},

	resetAndAddMusicList: async (musicList: MusicData[]) => {
		MusicController.pause();
		await MusicController.sendCommandController("clear");
		MusicController.stopProgress();
		MusicController.resetProgress();

		if (MusicController.currentMusicIndex() >= 0)
			MusicController.setCurrentMusicIndex(0);
		musicReset.set(true);
		await MusicController.addMusicList(musicList, {
			resetPlaylist: true,
		});
		musicReset.set(false);
	},

	onPlayerBarChange: () => {
		if (
			MusicController.isProgressValueEnd() ||
			MusicController.currentMusicIndex() < 0
		) {
			MusicController.setProgressValue(0);
			return;
		}

		setTimeout(() =>
			MusicController.sendCommandSetPosition(
				MusicController.realProgressDuration(),
			),
		);
	},

	updateProgressByPercentage: (percentage: number) => {
		if (
			MusicController.isProgressValueEnd() ||
			MusicController.currentMusicIndex() < 0
		) {
			MusicController.setProgressValue(0);
			return;
		}

		setTimeout(() => {
			if (isMobile())
				MusicController.setProgressValue(
					MusicController.parseProgressPercentageIntoValue(percentage),
				);
			MusicController.sendCommandSetPosition(
				MusicController.currentMusicRealDuration() * (percentage / 100),
			);
		});
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
		if (!playlist) playlist = MusicController.musicPlaylist();
		if (playlist.length < 2) return;

		for (let i = playlist.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[playlist[i], playlist[j]] = [playlist[j], playlist[i]];
		}
		await MusicController.resetAndAddMusicList(playlist);
		MusicController.play();
	},
	playlistMoveto: async (fromIndex: number, toIndex: number) => {
		if (fromIndex === toIndex) return;

		const currentMusic = MusicController.currentMusic();
		const playlist = MusicController.musicPlaylist();
		const music = playlist[fromIndex];
		playlist.splice(fromIndex, 1);
		playlist.splice(toIndex, 0, music);

		await invoke(CommandRoutes.MUSIC_PLAYLIST_MOVETO, {
			from: fromIndex,
			to:
				isDesktop() && fromIndex < MusicController.currentMusicIndex()
					? toIndex + 1
					: toIndex,
		});
		musicPlaylist.set(playlist);
		musicCurrentIndex.set(
			playlist.findIndex((m) => m.path === currentMusic.path),
		);
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
		await MusicController.setEqualizer(values);
	},

	getVisualizerBuffer: async (path: string) => {
		return await invoke<ArrayBuffer | null>(CommandRoutes.MUSIC_GET_VISUALIZER_BUFFER, {
			path,
		});
	},
};

export default MusicController;
