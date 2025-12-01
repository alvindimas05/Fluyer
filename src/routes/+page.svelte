<script lang="ts">
import {isMobile} from "$lib/platform";
import mobileStore from "$lib/stores/mobile.svelte";
import filterStore from "$lib/stores/filter.svelte";
import musicStore from "$lib/stores/music.svelte";
import {MusicListType} from "$lib/features/music/types";
import Intro from "$lib/features/intro/components/Intro.svelte";
import AlbumList from "$lib/features/album/components/AlbumList.svelte";
import MusicQueueList from "$lib/features/music_queue/components/MusicQueueList.svelte";
import CollectionInfo from "$lib/features/collection/components/CollectionInfo.svelte";
import MusicList from "$lib/features/music/components/MusicList.svelte";
import Menu from "$lib/features/menu/components/Menu.svelte";
import PlayerBar from "$lib/features/playerbar/components/PlayerBar.svelte";

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

{#if musicStore.list === null}
    <Intro />
{:else if Array.isArray(musicStore.list)}
    <!--{#if isDesktop()}-->
    <!--    <Equalizer />-->
    <!--{/if}-->
    <MusicQueueList />
    <Menu />
    <div class="h-full grid {gridClass}"
        style="padding-top: {paddingTop}px;">
        {#if [MusicListType.All, MusicListType.Album, MusicListType.Playlist].includes(musicStore.listType)}
            <AlbumList />
        {/if}
        {#if [MusicListType.All, MusicListType.Folder, MusicListType.Playlist].includes(musicStore.listType)}
            <CollectionInfo />
        {/if}
        {#if [MusicListType.All,  MusicListType.Music, MusicListType.Folder, MusicListType.Playlist].includes(musicStore.listType)}
        <MusicList />
        {/if}
    </div>
    <PlayerBar />
{/if}
