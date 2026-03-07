<script lang="ts">
	import { PageRoutes } from '$lib/constants/PageRoutes';
	import { IconType } from '$lib/ui/icon/types';
	import Sidebar from '$lib/features/sidebar/components/Sidebar.svelte';
	import MenuButton from '$lib/features/menu/components/MenuButton.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import PageService from '$lib/services/PageService.svelte';
	import MenuVolume from './MenuVolume.svelte';
	import { isDesktop, isWindows } from '$lib/platform';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/core';
	import { CommandRoutes } from '$lib/constants/CommandRoutes';
	import musicStore from '$lib/stores/music.svelte';
	import { MusicListType } from '$lib/features/music/types';
	import playlistStore from '$lib/stores/playlist.svelte';
	import TauriPlaylistAPI from '$lib/tauri/TauriPlaylistAPI';
	import folderStore from '$lib/stores/folder.svelte';
	import filterStore from '$lib/stores/filter.svelte';

	async function gotoPlayPage() {
		if (isDesktop()) {
			await getCurrentWindow().setFullscreen(true);
			if (isWindows()) {
				await invoke(CommandRoutes.RESTORE_ANIMATED_BACKGROUND);
			}
		}
		PageService.goTo(PageRoutes.PLAY);
	}
</script>

<Sidebar type={SidebarType.Left}>
	<p class="px-3 py-2 text-[1.2rem] font-semibold md:text-[1.5rem]">Menu</p>

	{#if musicStore.listType === MusicListType.Playlist}
		<MenuButton
			label="Music"
			icon={IconType.MusicListTypeMusic}
			onclick={() => {
				musicStore.listType = MusicListType.All;
				PageService.goTo(PageRoutes.HOME);
			}}
		/>
	{:else}
		<MenuButton
			label="Playlists"
			icon={IconType.MusicListTypePlaylist}
			onclick={async () => {
				try {
					playlistStore.list = await TauriPlaylistAPI.getAll();
				} catch (e) {
					console.error('Failed to load playlists:', e);
				}
				folderStore.currentFolder = null;
				filterStore.album = null;
				musicStore.listType = MusicListType.Playlist;
				PageService.goTo(PageRoutes.HOME);
			}}
		/>
	{/if}

	<MenuButton label="Play Screen" icon={IconType.Fullscreen} onclick={gotoPlayPage} />
	<!--{#if isDesktop() && !$settingBitPerfectMode}-->
	<!--    <MenuButton label="Equalizer" icon={IconType.Equalizer}-->
	<!--              onclick={() => UIController.toggleEqualizer(true)}/>-->
	<!--{/if}-->
	<!-- <MenuButton
		label="Visualizer"
		icon={IconType.Visualizer}
		onclick={() => PageService.goTo(PageRoutes.VISUALIZER)}
	/> -->
	<MenuButton
		label="Settings"
		icon={IconType.Settings}
		onclick={() => PageService.goTo(PageRoutes.SETTINGS)}
	/>
	<MenuVolume />
</Sidebar>
