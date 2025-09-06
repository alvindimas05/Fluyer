import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/commands";
import UtilsController from "$lib/controllers/UtilsController";
import {folderCurrent, folderList} from "$lib/stores/folder";
import {get} from "svelte/store";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import {type FolderData, type MusicData, MusicSize} from "$lib/home/music/types";
import {musicList} from "$lib/stores/music";
import MusicController, {MusicConfig} from "$lib/controllers/MusicController";
import { isWindows } from "$lib/platform";

const pathSeparator = isWindows() ? '\\' : '/';
const FolderController = {
    pathSeparator,
    initialize: async () => {
        await FolderController.setFolderList();
    },
    currentFolder: () => get(folderCurrent),
    setFolderList: async () => {
        const folder = FolderController.currentFolder();
        if (folder){
            folderList.set(await invoke(CommandRoutes.FOLDER_GET_ITEMS, { path: folder.path }));
        } else {
            folderList.set((await PersistentStoreController.musicPath.get())
                .map((path) => ({path} as FolderData)));
        }
    },
    setMusicListToFolder: async () => {
        const musicPaths = await PersistentStoreController.musicPath.get();
        console.log(musicPaths);
        if(musicPaths.length < 2) folderCurrent.set({path: musicPaths[0]} as FolderData);
        else folderCurrent.set(null);
        FolderController.setFolderList();
    },
    setFolder: async (folder: FolderData | null) => {
        folderCurrent.set(folder);
        await FolderController.setFolderList();
    },
    setFolderToParent: (folder: FolderData | null) => {
        if (!folder) return;
        const parent = folder.path.split(pathSeparator).slice(0, -1).join(pathSeparator);
        FolderController.setFolder({path: parent} as FolderData);
    },
    isMusicInFolder: (music: MusicData, folder: FolderData | null) => {
        if (!folder || !music.path.startsWith(folder.path)) return false;

        // Ensure folder path has a trailing slash for correct comparison
        const folderPathWithSlash = folder.path.endsWith(pathSeparator) ? folder.path : `${folder.path}${pathSeparator}`;
        const remainingPath = music.path.startsWith(folderPathWithSlash) ? music.path.substring(folderPathWithSlash.length) : '';

        // If the remaining path has no more slashes, it's in the immediate folder
        return remainingPath !== '' && !remainingPath.includes(pathSeparator);
    },
    isMusicInFolderRecursive: (music: MusicData, folder: FolderData | null) => folder && music.path.startsWith(folder.path),
    getImageFromPath: async (path: string, size: MusicSize | null) => {
        const imageSize = size ? size.toString() : null;
        const base64 =  await invoke<string>(CommandRoutes.FOLDER_GET_IMAGE, { path, size: imageSize });
        if (base64) return UtilsController.withBase64(base64);
        return MusicConfig.defaultAlbumImage;
    },
    getMusicListFromFolder: (folder: FolderData | null) => {
        if (!folder) return [];
        return MusicController.sortMusicList(
            get(musicList)!!.filter((music) =>
                FolderController.isMusicInFolderRecursive(music, folder))
        );
    },
};

export default FolderController;