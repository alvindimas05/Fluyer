<script lang="ts">
import Playlist from "$lib/home/playlist/Playlist.svelte";
import { musicList } from "$lib/stores/music";
import Intro from "$lib/home/intro/Intro.svelte";
import Menu from "$lib/home/menu/Menu.svelte";
import AlbumList from "$lib/home/album/AlbumList.svelte";
import MusicList from "$lib/home/music/MusicList.svelte";
import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
import { loadingShow } from "$lib/stores/loading";
import AlbumInfo from "$lib/home/albuminfo/AlbumInfo.svelte";
</script>
{#if $loadingShow}
    {#if $musicList === null}
        <Intro />
    {:else if Array.isArray($musicList)}
        <Playlist />
        <Menu />
        <PlayerBar />
        <div class="h-full grid grid-rows-[min-content_min-content_auto]">
            <AlbumList />
            <AlbumInfo />
            <MusicList />
        </div>
    {/if}
{/if}
