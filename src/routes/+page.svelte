<script lang="ts">
    import AlbumList from "$lib/home/album/AlbumList.svelte";
    import MusicList from "$lib/home/music/MusicList.svelte";
    import type { MusicData } from "$lib/home/music/types";
    import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
    import Playlist from "$lib/home/playlist/Playlist.svelte";
    import Loading from "$lib/loading/Loading.svelte";
    import LoadingController from "$lib/loading/LoadingController";
    import { loadingShow } from "$lib/loading/stores";
    import MusicController from "$lib/MusicController";
    
    let isLoadingDone = LoadingController.loadingShow();
        
    loadingShow.subscribe(() => {
       isLoadingDone = LoadingController.loadingShow(); 
    });
    
    LoadingController.listen();
    MusicController.getMusics();
</script>
{#if isLoadingDone}
<PlayerBar />
<div class="h-full grid grid-rows-[min-content_1fr]">
    <AlbumList />
    <MusicList />
    <Playlist />
</div>
{:else}
<Loading />
{/if}
