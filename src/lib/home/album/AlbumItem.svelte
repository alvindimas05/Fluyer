<script lang="ts">
    import SpotifyApi from "$lib/api/spotify";
    import { onMount } from "svelte";
    import type { MusicData } from "../music/types";

    interface Props {
        music: MusicData;
    }

    let { music }: Props = $props();
    
    const spotifyApi = new SpotifyApi();
    let albumImage = $state(`data:image/png;base64,${music.image}`);
    
    async function checkAlbumImage(){
        if(music.image !== null) return;
        const spotifyMusic = await spotifyApi.searchMusic(music);
        albumImage = spotifyMusic?.imageUrl
    }
    
    onMount(checkAlbumImage);
</script>

<div class="px-3 py-6 text-white row-[1] col-auto">
    <img class="rounded-lg" src={albumImage} alt="Album">
    <p class="font-medium text-xl mt-2">{music.album}</p>
    <p class="text-lg text-gray-300">{music.album_artist ?? music.artist}</p>
</div>