import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';
import { type FolderData, type MusicData } from '$lib/features/music/types';
import { isWindows } from '$lib/platform';
import folderStore from '$lib/stores/folder.svelte';
import musicSvelte from '$lib/stores/music.svelte';
import LibraryService from '$lib/services/LibraryService.svelte';
import PersistentStoreService from '$lib/services/PersistentStoreService.svelte';

const PATH_SEPARATOR = isWindows() ? '\\' : '/';

const FolderService = {
	PATH_SEPARATOR,

	initialize: async () => {
		FolderService.listenFolderEvents();
	},

	listenFolderEvents: () => {
		$effect(
			() =>
				void (async () => {
					const currentFolder = folderStore.currentFolder;
					let folders: FolderData[];

					if (currentFolder) {
						folders = await invoke(CommandRoutes.FOLDER_GET_ITEMS, {
							path: currentFolder.path
						});
					} else {
						const musicPaths = await PersistentStoreService.musicPath.get();
						folders = musicPaths.map((path) => ({ path }) as FolderData);
					}

					folders.sort((a, b) => a.path.localeCompare(b.path, undefined, { sensitivity: 'base' }));

					folderStore.list = folders;
				})()
		);
	},

	navigateToParent: async (folder: FolderData | null) => {
		if (!folder) return;

		const path = folder.path.split(PATH_SEPARATOR).slice(0, -1).join(PATH_SEPARATOR);

		folderStore.currentFolder = { path } as FolderData;
	},

	containsMusic: (music: MusicData, folder: FolderData | null): boolean => {
		if (!folder || !music.path.startsWith(folder.path)) return false;

		const folderPathWithSlash = folder.path.endsWith(PATH_SEPARATOR)
			? folder.path
			: `${folder.path}${PATH_SEPARATOR}`;

		// Music must start with folder path + separator
		if (!music.path.startsWith(folderPathWithSlash)) return false;

		const remainingPath = music.path.substring(folderPathWithSlash.length);

		// Check if music is in immediate folder (no nested folders)
		return remainingPath !== '' && !remainingPath.includes(PATH_SEPARATOR);
	},

	containsMusicRecursive: (music: MusicData, folder: FolderData | null): boolean => {
		return !!folder && music.path.startsWith(folder.path);
	},

	getMusicList: (folder: FolderData | null): MusicData[] => {
		if (!folder) return [];

		const filteredMusic = musicSvelte.list?.filter((music) =>
			FolderService.containsMusicRecursive(music, folder)
		);

		if (!filteredMusic) return [];

		return LibraryService.sortMusicList(filteredMusic);
	}
};

export default FolderService;
