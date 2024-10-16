<script lang="ts">
    import Album from "$lib/home/Album.svelte";
    import PlayerBar from "$lib/home/PlayerBar.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    
    onMount(async () => {
        const musics = await invoke('music_get_all');
        console.log(musics);
    });
</script>
<PlayerBar />
<div class="grid grid-cols-5 p-5">
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
    <Album />
</div>
