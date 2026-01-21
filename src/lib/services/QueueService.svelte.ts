import TauriMusicAPI, { TauriMusicCommand } from '$lib/tauri/TauriMusicAPI';
import musicStore from '$lib/stores/music.svelte';
import TauriQueueAPI from '$lib/tauri/TauriQueueAPI';
import { isDesktop } from '$lib/platform';
import type { MusicData } from '$lib/features/music/types';

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
		const newIds = list.map(() => crypto.randomUUID());
		musicStore.queueIds.push(...newIds);
	},
	resetAndAdd: (music: MusicData) => {
		return QueueService.resetAndAddList([music]);
	},
	resetAndAddList: async (list: MusicData[]) => {
		await TauriMusicAPI.sendCommand(TauriMusicCommand.Clear);

		// Generate new UUIDs for all items
		const newIds = list.map(() => crypto.randomUUID());
		musicStore.queueIds = newIds;
		musicStore.queue = list;
		await TauriQueueAPI.add(list);
	},
	goTo: (index: number) => {
		return TauriQueueAPI.goTo(index);
	},
	moveTo: (from: number, to: number) => {
		// Synchronous update of local state
		if (from === to) return Promise.resolve();

		// Calculate apiTo based on current state BEFORE mutation
		const apiTo = isDesktop() && from < musicStore.currentIndex ? to + 1 : to;

		const queue = [...musicStore.queue];
		const currentPath = musicStore.currentMusic?.path;

		const music = queue[from];
		queue.splice(from, 1);
		queue.splice(to, 0, music);

		// Commit synchronous update
		musicStore.queue = queue;
		if (currentPath) {
			const newIndex = queue.findIndex((m) => m.path === currentPath);
			if (newIndex !== -1) {
				musicStore.currentIndex = newIndex;
			}
		}

		// Queue backend API call mechanism
		return playlistMoveQueue.add(async () => {
			await TauriQueueAPI.moveTo(from, apiTo);
		});
	}
};

export default QueueService;
