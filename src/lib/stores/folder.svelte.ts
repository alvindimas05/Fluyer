import {type FolderData} from "$lib/features/music/types";

const folderStore = $state({
    list: [] as FolderData[],
    currentFolder: undefined as FolderData | undefined,
});

export default folderStore;