import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import FolderService from "$lib/services/FolderService.svelte";
import LibraryService from "$lib/services/LibraryService.svelte";
import {MusicConfig} from "$lib/constants/music";
import ProgressService from "$lib/services/ProgressService.svelte";
import folderSvelte from "$lib/stores/folder.svelte";
import QueueService from "$lib/services/QueueService.svelte";
import MusicPlayerService from "$lib/services/MusicPlayerService.svelte";
import ToastService from "$lib/services/ToastService.svelte";
import filterStore from "$lib/stores/filter.svelte";
import musicStore from "$lib/stores/music.svelte";
import {MusicListType} from "$lib/features/music/types";
import folderStore from "$lib/stores/folder.svelte";

export default function useCollectionInfo(){
    let album = $derived(filterStore.album);
    let showBackButton = $derived.by(async () => {
        const isNotFolderView = musicStore.listType !== MusicListType.Folder;

        const folderPath = folderStore.currentFolder?.path;
        const storedPath = await PersistentStoreService.musicPath.get();
        const isOutsideStoredPath = folderPath
            ? !storedPath.includes(folderPath)
            : false;

        return (
            isNotFolderView ||
            (storedPath.length === 1 && isOutsideStoredPath) ||
            storedPath.length > 1
        );
    });

    let musicList = $derived.by(() => {
        if (musicStore.listType === MusicListType.Folder) {
            return FolderService.getMusicList(folderStore.currentFolder);
        } else if (album) {
            return LibraryService.sortMusicList(album.musicList);
        }
        return [];
    });

    let label = $derived.by(() => {
        if (musicStore.listType === MusicListType.Folder && folderStore.currentFolder) {
            const folderMusic = FolderService.getMusicList(folderStore.currentFolder);
            const totalDuration = folderMusic.reduce(
                (acc, music) => acc + music.duration,
                0,
            );

            return `${folderStore.currentFolder.path} ${MusicConfig.separator} ${folderMusic.length} ${MusicConfig.separator} ${ProgressService.formatDuration(totalDuration)}`;
        } else if (album) {
            return [album.name, album.artist, album.year, album.duration]
                .filter(Boolean)
                .join(` ${MusicConfig.separator} `);
        }
        return null;
    });

    async function handleBack() {
        if (musicStore.listType === MusicListType.Folder) {
            const musicPaths = await PersistentStoreService.musicPath.get();
            if (musicPaths.includes(folderStore.currentFolder!!.path)) {
                folderSvelte.currentFolder = null;
                return;
            }
            await FolderService.navigateToParent(folderStore.currentFolder)
        } else {
            filterStore.album = null
        }
    }

    async function addMusicListAndPlay() {
        await QueueService.resetAndAddList(musicList);
        if(!musicStore.isPlaying) MusicPlayerService.play();
    }

    async function addMusicList() {
        await QueueService.addList(musicList);
        const label =
            musicStore.listType === MusicListType.Folder && folderStore.currentFolder
                ? folderStore.currentFolder.path
                : album
                    ? `${album.name} ${MusicConfig.separatorAlbum} ${album.artist}`
                    : null;
        ToastService.info(`Added music list to queue: ${label}`);
    }

    function playShuffle() {
        return MusicPlayerService.playShuffle(musicList);
    }

    return {
        album,
        label,
        showBackButton,
        handleBack,
        addMusicListAndPlay,
        addMusicList,
        playShuffle,
    };
}