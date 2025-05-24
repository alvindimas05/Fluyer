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
} from "$lib/stores/music";
import { get } from "svelte/store";
import type { MusicPlayerSync, MusicData } from "$lib/home/music/types";
import LoadingController from "$lib/controllers/LoadingController";
import { listen } from "@tauri-apps/api/event";
import { CommandRoutes } from "$lib/commands";
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
	separator: "•",
	defaultTitle: "The Meaning of Life",
	defaultArtist: "Musician",
	defaultAlbumImage: "/icons/default/default-album-cover.jpg",
	defaultPlayButton: "/icons/default/play.svg",
	defaultPauseButton: "/icons/default/pause.svg",
	defaultPreviousButton: "/icons/default/previous.svg",
	defaultNextButton: "/icons/default/next.svg",
	defaultPlaylistRemoveButton: "/icons/default/remove.svg",
	defaultPlayingIcon: "/icons/default/playing.svg",
	defaultNoteIcon: "/icons/default/note.svg",
	defaultBackButton: "/icons/default/back.svg",
	defaultSpeakerButton: "/icons/default/speaker.svg",
	defaultMuteButton: "/icons/default/mute.svg",
};
const MusicController = {
	initialize: () => {
		MusicController.listenSyncMusic();
		MusicController.handleVolumeChange();
	},
	musicList: () => get(musicList),
	setMusicList: (value: MusicData[] | null) => musicList.set(value),
	getMusics: async () => {
		if (MusicController.musicList()?.length) return;
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
	getAlbumImageFromMusic: (music: MusicData | null) => {
		if (music === null || music.image === null || music.image.length < 1)
			return MusicConfig.defaultAlbumImage;
		if (music.image === MusicConfig.defaultAlbumImage) return music.image;
		try {
			new URL(music.image);
			return music.image;
		} catch (e) {
			if (music.image.startsWith("data:image/png;base64,")) return music.image;
			return music.image
				? `data:image/png;base64, ${music.image}`
				: MusicConfig.defaultAlbumImage;
		}
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

	previousMusic: () => {
		if (MusicController.currentMusicIndex() <= 0) return;
		MusicController.gotoPlaylist(MusicController.currentMusicIndex() - 1);
	},

	nextMusic: async () => {
		MusicController.sendCommandController("next");
	},

	listenSyncMusic: () => {
		listen<MusicPlayerSync>(CommandRoutes.MUSIC_PLAYER_SYNC, async (e) => {
			if (e.payload.index != MusicController.currentMusicIndex()) {
				MusicController.setCurrentMusicIndex(e.payload.index);
			}
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
		const previousMusic = MusicController.currentMusic();
		if (
			previousMusic === null ||
			(previousMusic !== null && MusicController.isCurrentMusicFinished())
		) {
			musicIsPlaying.set(true);
			MusicController.startProgress({ resetProgress: false });
			if (sendCommand) MusicController.sendCommandController("play");
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

	addSinkMusic: async (path: string) =>
		await MusicController.addSinkMusics([path]),
	addSinkMusics: async (musicPaths: string[]) => {
		await invoke(CommandRoutes.MUSIC_PLAYLIST_ADD, {
			playlist: musicPaths,
		});
	},

	addMusicList: async (musicDataList: MusicData[]) => {
		if (MusicController.isCurrentMusicFinished() && musicDataList.length > 0) {
			MusicController.setCurrentMusicIndex(0);
		}

		let musicPaths: string[] = [];
		musicDataList.forEach((music) => musicPaths.push(music.path));
		MusicController.addSinkMusics(musicPaths);

		MusicController.addMusicPlaylist(musicDataList);
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
				if (a.trackNumber?.includes("/") || b.trackNumber?.includes("/")) {
					a.trackNumber = a.trackNumber!.split("/")[0];
					b.trackNumber = b.trackNumber!.split("/")[0];
				}
				return +a.trackNumber! - +b.trackNumber!;
			} else {
				return a.filename.localeCompare(b.filename);
			}
		});
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
};

export default MusicController;
