import TauriMusicAPI, { TauriMusicCommand } from '$lib/tauri/TauriMusicAPI';
import musicStore from '$lib/stores/music.svelte';
import TauriQueueAPI from '$lib/tauri/TauriQueueAPI';
import { isDesktop } from '$lib/platform';
import type { MusicData } from '$lib/features/music/types';
import * as uuid from 'uuid';

class PlaylistMoveQueue {
	private queue: Array<() => Promise<void>> = [];
	private isProcessing = false;

	async add(operation: () => Promise<void>): Promise<void> {
		return new Promise((resolve, reject) => {
			this.queue.push(async () => {
				try {
					await operation();
					resolve();
				} catch (error) {
					reject(error);
				}
			});

			// Start processing if not already running
			if (!this.isProcessing) {
				this.processQueue();
			}
		});
	}

	private async processQueue(): Promise<void> {
		if (this.isProcessing || this.queue.length === 0) return;

		this.isProcessing = true;

		while (this.queue.length > 0) {
			const operation = this.queue.shift();
			if (operation) {
				try {
					await operation();
				} catch (error) {
					console.error('Queue operation failed:', error);
				}
			}
		}

		this.isProcessing = false;
	}

	getQueueLength(): number {
		return this.queue.length;
	}
}

const playlistMoveQueue = new PlaylistMoveQueue();

const QueueService = {
	add: (music: MusicData) => {
		return QueueService.addList([music]);
	},
	remove: (index: number) => {
		musicStore.queue.splice(index, 1);
	},
	addList: async (list: MusicData[]) => {
		await TauriQueueAPI.add(list);
		musicStore.queue.push(...list);
		// Generate new UUIDs for added items
		const newIds = list.map(() => uuid.v4());
		musicStore.queueIds.push(...newIds);
	},
	resetAndAdd: (music: MusicData) => {
		return QueueService.resetAndAddList([music]);
	},
	resetAndAddList: async (list: MusicData[]) => {
		await TauriMusicAPI.sendCommand(TauriMusicCommand.Clear);

		// Generate new UUIDs for all items
		const newIds = list.map(() => uuid.v4());
		musicStore.queueIds = newIds;
		musicStore.queue = list;
		await TauriQueueAPI.add(list);
	},
	goTo: (index: number) => {
		return TauriQueueAPI.goTo(index);
	},
	moveTo: (from: number, to: number) => {
		if (from === to) return Promise.resolve();

		// Calculate apiTo BEFORE any state mutation to avoid stale index
		const apiTo = isDesktop() && from < musicStore.currentIndex ? to + 1 : to;

		// Update both queue and queueIds atomically
		const queue = [...musicStore.queue];
		const queueIds = [...musicStore.queueIds];
		const currentPath = musicStore.currentMusic?.path;

		const music = queue[from];
		const uuid = queueIds[from];

		queue.splice(from, 1);
		queue.splice(to, 0, music);

		queueIds.splice(from, 1);
		queueIds.splice(to, 0, uuid);

		// Commit both updates atomically
		musicStore.queue = queue;
		musicStore.queueIds = queueIds;

		if (currentPath) {
			const newIndex = queue.findIndex((m) => m.path === currentPath);
			if (newIndex !== -1) {
				console.log('Moving to index:', newIndex);
				musicStore.currentIndex = newIndex;
			}
		}

		// Queue backend API call
		return playlistMoveQueue.add(async () => {
			await TauriQueueAPI.moveTo(from, apiTo);
		});
	},
	clear: async () => {
		await TauriMusicAPI.sendCommand(TauriMusicCommand.Clear);
		musicStore.queue = [];
		musicStore.queueIds = [];
		musicStore.currentIndex = -1;
	}
};

export default QueueService;
