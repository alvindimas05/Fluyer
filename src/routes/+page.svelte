<script lang="ts">
import AlbumList from "$lib/home/album/AlbumList.svelte";
import MusicList from "$lib/home/music/MusicList.svelte";
import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
import Playlist from "$lib/home/playlist/Playlist.svelte";
import LoadingController from "$lib/controllers/LoadingController";
import { loadingBackground, loadingShow } from "$lib/stores/loading";
import MusicController from "$lib/controllers/MusicController";
import { musicList } from "$lib/stores/music";
import { isMobile } from "$lib/platform";
import Intro from "$lib/home/intro/Intro.svelte";
import SearchBar from "$lib/searchbar/SearchBar.svelte";

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
        <SearchBar />
        <PlayerBar />
        <Playlist />
        <div class="h-full grid grid-rows-[min-content_auto]">
            <AlbumList />
            <MusicList />
        </div>
    {/if}
{/if}
