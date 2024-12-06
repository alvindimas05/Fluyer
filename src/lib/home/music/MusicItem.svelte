<script lang="ts">
    import { onMount } from "svelte";
    import type { MusicData } from "./types";
    import SpotifyApi from "$lib/api/spotify";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { musicIsPlaying, musicPlayed } from "$lib/stores/music";
    import { album } from "$lib/stores";

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
        await invoke("music_playlist_add", { playlist: [music.path] });
        const unlisten = await listen("music_playlist_add", async (_) => {
            invoke("music_controller", { command: "play" });
            unlisten();
            $musicPlayed = music;
            $musicIsPlaying = true;
        });
        $album = albumImage;
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
