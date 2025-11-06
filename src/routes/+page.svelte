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
import {isDesktop, isMobile} from "$lib/platform";
import {MusicListType} from "$lib/home/music/types";
import {mobileStatusBarHeight} from "$lib/stores/mobile";
import {filterBarHeight} from "$lib/stores/filterbar";

let paddingTop = $derived(
    (isMobile() ? $mobileStatusBarHeight : 0) + $filterBarHeight,
);

let gridClass = $derived.by(() => {
    switch($musicListType){
        case MusicListType.All: return 'grid-rows-[min-content_min-content_auto]';
        case MusicListType.Music: return '';
        default: return 'grid-rows-[min-content_auto]';
    }
})
</script>

{#if $loadingShow}
    {#if $musicList === null}
        <Intro />
    {:else if Array.isArray($musicList)}
        {#if isDesktop()}
            <Equalizer />
        {/if}
        <Playlist />
        <Menu />
        <div class="h-full grid {gridClass}"
            style="padding-top: {paddingTop}px;">
            {#if [MusicListType.All, MusicListType.Album].includes($musicListType)}
                <AlbumList />
            {/if}
            {#if ![MusicListType.Music, MusicListType.Album].includes($musicListType)}
                <AlbumInfo />
            {/if}
            {#if $musicListType !== MusicListType.Album}
                <MusicList />
            {/if}
        </div>
        <PlayerBar />
    {/if}
{/if}
