<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import View from "$lib/ui/components/View.svelte";
import filterStore from "$lib/stores/filter.svelte";
import musicStore from "$lib/stores/music.svelte";
import {MusicListType} from "$lib/features/music/types";
import folderStore from "$lib/stores/folder.svelte";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import FolderService from "$lib/services/FolderService.svelte";
import LibraryService from "$lib/services/LibraryService.svelte";
import {MusicConfig} from "$lib/constants/music";
import ProgressService from "$lib/services/ProgressService.svelte";
import folderSvelte from "$lib/stores/folder.svelte";
import QueueService from "$lib/services/QueueService.svelte";
import MusicPlayerService from "$lib/services/MusicPlayerService.svelte";
import ToastService from "$lib/services/ToastService.svelte";

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
</script>
{#if album || folderStore.currentFolder || musicStore.listType === MusicListType.Playlist}
    <View class="mx-3 mb-2 md:grid grid-cols-[auto_max-content] px-4 py-2 rounded-lg
        box-border animate__animated animate__fadeIn
        hover:px-5 hover:py-3">
        <div class="grid items-center">
            <div class="text-sm md:text-base font-medium overflow-hidden">
                <p class="whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text">{label}</p>
            </div>
        </div>
        <div class="w-fit">
            <div>

            </div>
            {#await showBackButton then showBackButton}
                <div class="grid gap-x-2 md:gap-x-3 mt-2 md:mt-0"
                     class:grid-cols-4={showBackButton}
                     class:grid-cols-3={!showBackButton}>
                    {#if showBackButton}
                        <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                                onclick={handleBack}>
                            <Icon type={IconType.AlbumBack} />
                        </button>
                    {/if}
                    <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                            onclick={addMusicListAndPlay}>
                        <Icon type={IconType.Play} />
                    </button>
                    <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                            onclick={addMusicList}>
                        <Icon type={IconType.QueuePlaylist} />
                    </button>
                    <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                            onclick={playShuffle}>
                        <Icon type={IconType.Shuffle} />
                    </button>
                </div>
            {/await}
        </div>
    </View>
{:else}
    <div></div>
{/if}