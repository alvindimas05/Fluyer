<script lang="ts">
import Playlist from "$lib/home/playlist/Playlist.svelte";
import {musicList, musicListType} from "$lib/stores/music";
import Intro from "$lib/home/intro/Intro.svelte";
import Menu from "$lib/home/menu/Menu.svelte";
import AlbumList from "$lib/home/album/AlbumList.svelte";
import MusicList from "$lib/home/music/MusicList.svelte";
import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
import {loadingShow} from "$lib/stores/loading";
import AlbumInfo from "$lib/home/albuminfo/AlbumInfo.svelte";
import Equalizer from "$lib/home/equalizer/Equalizer.svelte";
import {isDesktop} from "$lib/platform";
import {MusicListType} from "$lib/home/music/types";
import {MusicConfig} from "$lib/controllers/MusicController";
import {onDestroy, onMount} from "svelte";

let filterBarHeight = $state(MusicConfig.filterBarHeight);

function updateFilterBarHeight() {
    if($musicListType !== MusicListType.Folder){
        filterBarHeight = 0;
        return;
    }

    filterBarHeight = MusicConfig.filterBarHeight * (window.innerWidth > 640 ? 1 : 2);
}

let unsubscribeMusicListType = musicListType.subscribe(() => setTimeout(updateFilterBarHeight));

onMount(() => {
    updateFilterBarHeight();
});

onDestroy(() => {
    unsubscribeMusicListType();
});
</script>

<svelte:window onresize={updateFilterBarHeight} />
{#if $loadingShow}
    {#if $musicList === null}
        <Intro />
    {:else if Array.isArray($musicList)}
        {#if isDesktop()}
            <Equalizer />
        {/if}
        <Playlist />
        <Menu />
        <PlayerBar />
        <div class="h-full grid {$musicListType === MusicListType.Folder ?
            'grid-rows-[min-content_auto]' : 'grid-rows-[min-content_min-content_auto]'}"
            style="margin-top: {filterBarHeight}px;">
            {#if $musicListType === MusicListType.All}
                <AlbumList />
            {/if}
            <AlbumInfo />
            <MusicList />
        </div>
    {/if}
{/if}
