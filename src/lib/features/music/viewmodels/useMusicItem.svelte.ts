import { type FolderData, type MusicData, MusicListType } from "../types";
import MetadataService from "$lib/services/MetadataService.svelte";
import folderStore from "$lib/stores/folder.svelte";
import FolderService from "$lib/services/FolderService.svelte";
import musicStore from "$lib/stores/music.svelte";
import {MusicConfig} from "$lib/constants/music";
import ProgressService from "$lib/services/ProgressService.svelte";
import QueueService from "$lib/services/QueueService.svelte";
import MusicPlayerService from "$lib/services/MusicPlayerService.svelte";
import ToastService from "$lib/services/ToastService.svelte";

export function useMusicItem(music: MusicData, folder?: FolderData) {
    const albumImage = $derived.by(async () => {
        return folder
            ? await MetadataService.getFolderCoverArt(folder.path)
            : await MetadataService.getMusicCoverArt(music);
    });

    const titleLabel = $derived.by(() => {
        if (folder) {
            return folderStore.currentFolder
                ? folder.path.split(FolderService.PATH_SEPARATOR).pop()
                : folder.path;
        }
        return musicStore.listType === MusicListType.Folder ? music.filename : music.title;
    });

    const mediumLabel = $derived.by(() => {
        if (folder) return "Folder";

        const album = music.album ? `${music.album} ${MusicConfig.separatorAlbum} ` : "";
        const artist = music.artist ?? MusicConfig.defaultArtist;
        return `${album}${artist}`;
    });

    const smallLabel = $derived.by(() => {
        if (folder) {
            const folderMusic = FolderService.getMusicList(folder);
            const totalDuration = folderMusic.reduce((acc, m) => acc + m.duration, 0);
            const durationText = ProgressService.formatDuration(totalDuration);
            return `${folderMusic.length} ${MusicConfig.separator} ${durationText}`;
        }

        const duration = ProgressService.formatDuration(music.duration);
        const resolution = [
            music.bitsPerSample && `${music.bitsPerSample}-bit`,
            MetadataService.formatSampleRate(music.sampleRate),
        ].filter(Boolean);

        if (!resolution.length) return duration;

        const audioResolution = resolution.join(MusicConfig.separatorAudio);
        return `${audioResolution} ${MusicConfig.separator} ${duration}`;
    });

    async function addMusicAndPlay() {
        if (music) {
            await QueueService.resetAndAdd(music);
        } else {
            await QueueService.resetAndAddList(
                FolderService.getMusicList(folder!)
            );
        }
        if (!musicStore.isPlaying){
            MusicPlayerService.play();
        }
    }

    async function addMusic() {
        const musicList = music
            ? [music]
            : FolderService.getMusicList(folder!);

        if (music) {
            await QueueService.add(music);
        } else {
            await QueueService.resetAndAddList(musicList);
        }

        const title = music.title ?? music.filename ?? MusicConfig.defaultTitle;
        const artist = music.artist ?? MusicConfig.defaultArtist;
        ToastService.info(
            `Added music to queue: ${title} ${MusicConfig.separatorAlbum} ${artist}`
        );
    }

    async function selectFolder() {
        if (folder) folderStore.currentFolder = folder;
    }

    return {
        get albumImage() {
            return albumImage;
        },
        get titleLabel() {
            return titleLabel;
        },
        get mediumLabel() {
            return mediumLabel;
        },
        get smallLabel() {
            return smallLabel;
        },
        addMusicAndPlay,
        addMusic,
        selectFolder,
    };
}