import {type FolderData} from "$lib/features/music/types";

const folderStore = $state({
    list: [] as FolderData[],
    currentFolder: null as FolderData | null,
});

export default folderStore;