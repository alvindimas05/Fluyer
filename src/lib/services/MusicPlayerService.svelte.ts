import musicStore from "$lib/stores/music.svelte";
import ProgressService from "$lib/services/ProgressService.svelte";
import TauriMusicAPI, {TauriMusicCommand} from "$lib/tauri/TauriMusicAPI";
import QueueService from "$lib/services/QueueService.svelte";
import musicSvelte from "$lib/stores/music.svelte";
import {RepeatMode} from "$lib/home/music/types";
import type {MusicData} from "$lib/features/music/types";

const MusicPlayerService = {
    play: () => {
        if (musicStore.queue.length === 0) return;

        musicStore.isPlaying = true
        ProgressService.start();
    },
    pause: () => {
        musicStore.isPlaying = true;
        ProgressService.stop();
    },
    next: () => {
        return TauriMusicAPI.sendCommand(TauriMusicCommand.Next);
    },
    previous: () => {
        return QueueService.goTo(musicSvelte.currentIndex - 1);
    },
    seekByPercentage: async (percentage: number) => {
        const clamped = Math.min(100, Math.max(0, percentage));
        const position = (musicStore.currentMusic?.duration ?? 0) * (clamped / 100);

        await TauriMusicAPI.setPosition(position);
        await TauriMusicAPI.requestSync();
    },
    toggleRepeatMode: ()=> {
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
    playShuffle: async (list?: MusicData[]) => {
        if(!list) list = musicStore.queue;
        for (let i = list.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [list[i], list[j]] = [list[j], list[i]];
        }
        await QueueService.resetAndAddList(list);
        if(!musicStore.isPlaying) MusicPlayerService.play();
    },
};

export default MusicPlayerService;