<script lang="ts">
import Playlist from "$lib/home/playlist/Playlist.svelte";
import Menu from "$lib/home/menu/Menu.svelte";
import AlbumList from "$lib/home/album/AlbumList.svelte";
import MusicList from "$lib/home/music/MusicList.svelte";
import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
import AlbumInfo from "$lib/home/albuminfo/AlbumInfo.svelte";
import Equalizer from "$lib/home/equalizer/Equalizer.svelte";
import {isDesktop, isMobile} from "$lib/platform";
import mobileStore from "$lib/stores/mobile.svelte";
import filterStore from "$lib/stores/filter.svelte";
import loadingStore from "$lib/stores/loading.svelte";
import musicStore from "$lib/stores/music.svelte";
import {MusicListType} from "$lib/features/music/types";
import Intro from "$lib/features/intro/components/Intro.svelte";

let paddingTop = $derived(
    (isMobile() ? mobileStore.statusBarHeight : 0) + filterStore.bar.height,
);

let gridClass = $derived.by(() => {
    switch(musicStore.listType){
        case MusicListType.All: return 'grid-rows-[min-content_min-content_auto]';
        case MusicListType.Music: return '';
        default: return 'grid-rows-[min-content_auto]';
    }
});
</script>

{#if loadingStore.show}
    {#if musicStore.list === null}
        <Intro />
    {:else if Array.isArray(musicStore.list)}
        {#if isDesktop()}
            <Equalizer />
        {/if}
        <Playlist />
        <Menu />
        <div class="h-full grid {gridClass}"
            style="padding-top: {paddingTop}px;">
            {#if [MusicListType.All, MusicListType.Album, MusicListType.Playlist].includes(musicStore.listType)}
                <AlbumList />
            {/if}
            {#if [MusicListType.All, MusicListType.Folder, MusicListType.Playlist].includes(musicStore.listType)}
                <AlbumInfo />
            {/if}
            {#if [MusicListType.All,  MusicListType.Music, MusicListType.Folder, MusicListType.Playlist].includes(musicStore.listType)}
                <MusicList />
            {/if}
        </div>
        <PlayerBar />
    {/if}
{/if}
