import musicStore from '$lib/stores/music.svelte';
import ProgressService from '$lib/services/ProgressService.svelte';
import TauriMusicAPI, { TauriMusicCommand } from '$lib/tauri/TauriMusicAPI';
import QueueService from '$lib/services/QueueService.svelte';
import { type MusicPlayerSync, RepeatMode } from '$lib/features/music/types';
import { listen } from '@tauri-apps/api/event';
import { CommandRoutes } from '$lib/constants/CommandRoutes';
import PersistentStoreService from '$lib/services/PersistentStoreService.svelte';
import { MusicConfig } from '$lib/constants/MusicConfig';

const MusicPlayerService = {
	initialize: async () => {
		MusicPlayerService.listenSyncEvents();
		MusicPlayerService.listenVolumeEvents();
	},
	play: async () => {
		if (musicStore.queue.length === 0) {
			console.warn("Can't play music playback because the queue is empty.");
			return;
		}

		if (musicStore.isPlaying) {
			console.warn('Playing music playback called but music is already played.');
		}

		console.log('Starting music playback...');

		musicStore.isPlaying = true;
		await TauriMusicAPI.sendCommand(TauriMusicCommand.Play);
		ProgressService.start();
	},
	pause: async () => {
		if (!musicStore.isPlaying) {
			console.warn('Pausing music playback called but music is already paused.');
		}

		console.log('Pausing music playback...');
		musicStore.isPlaying = false;
		await TauriMusicAPI.sendCommand(TauriMusicCommand.Pause);
		ProgressService.stop();
	},
	next: async () => {
		if (musicStore.currentIndex === musicStore.queue.length - 1) {
			// If we are at the end, do nothing (unless we want to repeat/loop, but request says "do nothing")
			// But wait, the existing code:
			// await MusicPlayerService.seekByPercentage(0);
			// await MusicPlayerService.pause();
			// ProgressService.reset();
			// return;
			// This was "stop at end". The request implies "do nothing" (don't even stop/seek?).
			// "if the user pressing previous ... do nothing".
			// So I should just return.
			return;
		}
		return TauriMusicAPI.sendCommand(TauriMusicCommand.Next);
	},
	previous: async () => {
		if (musicStore.currentIndex === 0) {
			return;
		}
		return QueueService.goTo(musicStore.currentIndex - 1);
	},
	seekByPercentage: async (percentage: number) => {
		const clamped = Math.min(100, Math.max(0, percentage));
		const position = (musicStore.currentMusic?.duration ?? 0) * (clamped / 100);

		await TauriMusicAPI.setPosition(position);
		await TauriMusicAPI.requestSync();
	},
	toggleRepeatMode: () => {
		let nextRepeatMode = RepeatMode.None;
		const currentMode = musicStore.repeatMode;
		switch (currentMode) {
			case RepeatMode.None:
				nextRepeatMode = RepeatMode.All;
				break;
			case RepeatMode.All:
				nextRepeatMode = RepeatMode.One;
				break;
			case RepeatMode.One:
				nextRepeatMode = RepeatMode.One;
				break;
		}
		musicStore.repeatMode = nextRepeatMode;
	},

	listenSyncEvents: () => {
		return listen<MusicPlayerSync>(CommandRoutes.MUSIC_PLAYER_SYNC, async (e) => {
			console.log('Received music player sync event:', e.payload);

			if (e.payload.index > -1) {
				musicStore.currentIndex = e.payload.index;
				musicStore.progressValue =
					(e.payload.currentPosition / musicStore.currentMusic!!.duration) * MusicConfig.max;
			} else ProgressService.reset();

			musicStore.isPlaying = e.payload.isPlaying;
			if (e.payload.isPlaying) {
				ProgressService.stop();
				ProgressService.start();
			} else ProgressService.stop();
		});
	},
	listenVolumeEvents: () => {
		$effect(() => {
			(async () => {
				await PersistentStoreService.volume.set(musicStore.volume);
				await TauriMusicAPI.setVolume(musicStore.volume);
			})();
		});
	}
};

export default MusicPlayerService;
