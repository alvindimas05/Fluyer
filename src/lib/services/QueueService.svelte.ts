import TauriMusicAPI, {TauriMusicCommand} from "$lib/tauri/TauriMusicAPI";
import ProgressService from "$lib/services/ProgressService.svelte";
import musicStore from "$lib/stores/music.svelte";
import TauriQueueAPI from "$lib/tauri/TauriQueueAPI";
import {playlistMoveQueue} from "$lib/home/playlist/PlaylistMoveQueue";
import {isDesktop} from "$lib/platform";
import type {MusicData} from "$lib/features/music/types";

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
    },
    resetAndAdd: (music: MusicData) => {
        return QueueService.resetAndAddList([music]);
    },
    resetAndAddList: async (list: MusicData[]) => {
        await TauriMusicAPI.sendCommand(TauriMusicCommand.Clear);
        ProgressService.start();

        musicStore.reset = true;
        await TauriQueueAPI.add(list);
        musicStore.queue = list;
        musicStore.reset = false;
    },
    goTo: (index: number) => {
        return TauriQueueAPI.goTo(index);
    },
    moveTo: (from: number, to: number) => {
        return playlistMoveQueue.add(async () => {
            if (from === to) return;

            const queue = musicStore.queue;
            const music = musicStore.queue[from];
            queue.splice(from, 1);
            queue.splice(to, 0, music);

            const apiTo = isDesktop() && from < musicStore.currentIndex
                ? to + 1 : to;
            await TauriQueueAPI.moveTo(from, apiTo);

            musicStore.queue = queue;
            musicStore.currentIndex = queue.findIndex((m) => m.path === musicStore.currentMusic?.path);
        });
    },
};

export default QueueService;