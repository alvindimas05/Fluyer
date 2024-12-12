<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "./types";
import SpotifyApi from "$lib/api/spotify";
import MusicController from "$lib/controllers/MusicController";

interface Props {
	music: MusicData;
}

let { music }: Props = $props();

const spotifyApi = new SpotifyApi();

let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null) return;
	const spotifyMusic = await spotifyApi.searchMusic(music);
	if (spotifyMusic == null) return;
	albumImage = spotifyMusic?.imageUrl;
}

async function addMusicAndPlay() {
	await MusicController.addMusic(music);
	MusicController.play();
}

onMount(checkAlbumImage);
</script>

<div class="grid grid-cols-[max-content_auto] py-2 animate__animated animate__fadeInDown animate__slow">
    <button onclick={addMusicAndPlay}
        ><img class="w-16 rounded" src={albumImage} alt="Album" /></button
    >
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-200">{MusicController.getFullArtistFromMusic(music)}</p>
    </div>
</div>
