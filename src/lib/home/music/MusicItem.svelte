<script lang="ts">
    import { onMount } from "svelte";
    import type { MusicData } from "./types";
    import SpotifyApi from "$lib/api/spotify";
    import MusicController from "$lib/MusicController";

    interface Props {
        music: MusicData;
    }

    let { music }: Props = $props();

    const spotifyApi = new SpotifyApi();
    const albumArtist =
        music.album_artist && !music.artist.includes(music.album_artist)
            ? ` • ${music.album_artist}`
            : "";

    let albumImage = $state(music.image ? `data:image/png;base64,${music.image}` : '/icons/default/default-album-cover.jpg');

    async function checkAlbumImage() {
        if (music.image !== null) return;
        const spotifyMusic = await spotifyApi.searchMusic(music);
        albumImage = spotifyMusic?.imageUrl;
    }

    async function addMusicAndPlay() {
        MusicController.addMusic(music.path);
        MusicController.setIsPlaying(true);
        
        music.image = albumImage;
        MusicController.setCurrentMusic(music);
    }

    onMount(checkAlbumImage);
</script>

<div class="grid grid-cols-[max-content_auto] py-2">
    <button onclick={addMusicAndPlay}
        ><img class="w-16 rounded" src={albumImage} alt="Album" /></button
    >
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-200">{music.artist}{albumArtist}</p>
    </div>
</div>
