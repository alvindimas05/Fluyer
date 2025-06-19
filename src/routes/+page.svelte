<script lang="ts">
import Playlist from "$lib/home/playlist/Playlist.svelte";
import LoadingController from "$lib/controllers/LoadingController";
import { loadingBackground, loadingShow } from "$lib/stores/loading";
import MusicController from "$lib/controllers/MusicController";
import { musicList } from "$lib/stores/music";
import Intro from "$lib/home/intro/Intro.svelte";
import Menu from "$lib/home/menu/Menu.svelte";
import AlbumList from "$lib/home/album/AlbumList.svelte";
import MusicList from "$lib/home/music/MusicList.svelte";
import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
import FilterBar from "$lib/filterbar/FilterBar.svelte";

let isLoadingDone = LoadingController.loadingShow();

loadingBackground.subscribe(() => {
	if (!LoadingController.loadingBackground()) return;
	MusicController.getMusics();
});
loadingShow.subscribe(() => {
	isLoadingDone = LoadingController.loadingShow();
});

LoadingController.listen();
</script>
{#if isLoadingDone}
    {#if $musicList === null}
        <Intro />
    {:else if Array.isArray($musicList)}
        <Playlist />
        <Menu />
        <PlayerBar />
        <div class="h-full grid grid-rows-[min-content_min-content_auto]">
            <FilterBar />
            <AlbumList />
            <MusicList />
        </div>
    {/if}
{/if}
