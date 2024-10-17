<script lang="ts">
    import AlbumList from "$lib/home/album/AlbumList.svelte";
    import MusicList from "$lib/home/music/MusicList.svelte";
    import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
</script>
<PlayerBar />
<div class="h-full grid grid-rows-[min-content_1fr]">
    <AlbumList />
    <MusicList />
</div>
