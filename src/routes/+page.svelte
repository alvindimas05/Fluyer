<script lang="ts">
    import AlbumList from "$lib/home/album/AlbumList.svelte";
    import MusicList from "$lib/home/music/MusicList.svelte";
    import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
    import Playlist from "$lib/home/playlist/Playlist.svelte";
    import Loading from "$lib/loading/Loading.svelte";
    import LoadingController from "$lib/controllers/LoadingController";
    import { loadingShow } from "$lib/stores/loading";
    import MusicController from "$lib/controllers/MusicController";
    import MusicDirSelector from "$lib/home/music/MusicDirSelector.svelte";

    // let isLoadingDone = LoadingController.loadingShow();

    // loadingShow.subscribe(() => {
    //     isLoadingDone = LoadingController.loadingShow();
    // });

    // LoadingController.listen();
    // MusicController.getMusics();
</script>

<!-- {#if isLoadingDone}
<PlayerBar />
<Playlist />
<div class="h-full grid grid-rows-[min-content_1fr]">
    <AlbumList />
    <MusicList />
</div>
{:else}
<Loading />
{/if} -->

<MusicDirSelector />
