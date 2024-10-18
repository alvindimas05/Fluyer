<script lang="ts">
    import AlbumList from "$lib/home/album/AlbumList.svelte";
    import MusicList from "$lib/home/music/MusicList.svelte";
    import type { MusicData } from "$lib/home/music/types";
    import PlayerBar from "$lib/home/playerbar/PlayerBar.svelte";
    import { musicList } from "$lib/home/stores/music";
    import { invoke } from "@tauri-apps/api/core";
    
    async function getMusics(){
        if($musicList.length > 0) return;
        $musicList = await invoke<MusicData[]>('music_get_all');
    }
    getMusics();
</script>
<PlayerBar />
<div class="h-full grid grid-rows-[min-content_1fr]">
    <AlbumList />
    <MusicList />
</div>
