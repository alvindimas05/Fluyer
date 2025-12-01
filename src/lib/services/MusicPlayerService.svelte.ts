import musicStore from "$lib/stores/music.svelte";
import musicSvelte from "$lib/stores/music.svelte";
import ProgressService from "$lib/services/ProgressService.svelte";
import TauriMusicAPI, {TauriMusicCommand} from "$lib/tauri/TauriMusicAPI";
import QueueService from "$lib/services/QueueService.svelte";
import {type MusicData, type MusicPlayerSync, RepeatMode} from "$lib/features/music/types";
import {listen} from "@tauri-apps/api/event";
import {CommandRoutes} from "$lib/commands";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import {MusicConfig} from "$lib/constants/music";

const MusicPlayerService = {
    initialize: async () => {
        MusicPlayerService.listenSyncEvents();
        MusicPlayerService.listenVolumeEvents();
    },
    play: () => {
        if (musicStore.queue.length === 0) return;

        musicStore.isPlaying = true
        return TauriMusicAPI.sendCommand(TauriMusicCommand.Play);
    },
    pause: () => {
        musicStore.isPlaying = true;
        return TauriMusicAPI.sendCommand(TauriMusicCommand.Pause);
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

    listenSyncEvents: () => {
        return listen<MusicPlayerSync>(CommandRoutes.MUSIC_PLAYER_SYNC, async (e) => {
            if (e.payload.isPlaying){
                ProgressService.reset();
                ProgressService.start();
            }
            else ProgressService.stop();

            if(e.payload.index > -1){
                musicStore.currentIndex = e.payload.index;
                musicStore.progressValue = (e.payload.currentPosition / musicStore.currentMusic!!.duration) * MusicConfig.max;
            } else {
                ProgressService.reset();
            }
            musicStore.isPlaying = e.payload.isPlaying;
        });
    },
    listenVolumeEvents: () => {
        $effect(() => {
            (async () => {
                await PersistentStoreService.volume.set(musicStore.volume);
                await TauriMusicAPI.setVolume(musicStore.volume);
            })();
        });
    },
};

export default MusicPlayerService;