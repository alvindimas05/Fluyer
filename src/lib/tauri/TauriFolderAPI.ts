import { invoke } from '@tauri-apps/api/core';
import { TauriCommands } from '$lib/constants/TauriCommands';
import type { FolderData } from '$lib/features/music/types';

const TauriFolderAPI = {
	getFolderItems: (path: string) => {
		return invoke<FolderData[]>(TauriCommands.FOLDER_ITEMS_GET, { path });
	}
};

export default TauriFolderAPI;
