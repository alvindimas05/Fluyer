<script lang="ts">
    import { onMount } from "svelte";
    import type { MusicData } from "./types";
    import SpotifyApi from "$lib/api/spotify";
    import MusicController from "$lib/Music";

    export let music: MusicData;

    const spotifyApi = new SpotifyApi();
    const albumArtist =
        music.album_artist && !music.artist.includes(music.album_artist)
            ? ` • ${music.album_artist}`
            : "";

    let albumImage = `data:image/png;base64,${music.image}`;

    async function checkAlbumImage() {
        if (music.image !== null) return;
        const spotifyMusic = await spotifyApi.searchMusic(music);
        albumImage = spotifyMusic?.imageUrl;
    }

    async function addMusicAndPlay() {
        MusicController.addMusic(music.path);
        MusicController.setIsPlaying(true);
        MusicController.setCurrentMusic(music);
    }

    onMount(checkAlbumImage);
</script>

<div class="grid grid-cols-[max-content_auto] py-2">
    <button on:click={addMusicAndPlay}
        ><img class="w-16 rounded" src={albumImage} alt="Album" /></button
    >
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-400">{music.artist}{albumArtist}</p>
    </div>
</div>
