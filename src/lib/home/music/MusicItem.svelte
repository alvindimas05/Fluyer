<script lang="ts">
    import { onMount } from "svelte";
    import type { MusicData } from "./types";
    import SpotifyApi from "$lib/api/spotify";
    import MusicController, {
        MusicConfig,
    } from "$lib/controllers/MusicController";

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
        music.image = albumImage;
        const previousMusic = MusicController.currentMusic();
        await MusicController.addMusic(music);
        if (
            previousMusic === null ||
            (!previousMusic !== null &&
                MusicController.isCurrentMusicFinished())
        )
            MusicController.play();
    }

    onMount(checkAlbumImage);
</script>

<div
    class="grid grid-cols-[max-content_auto] py-2 animate__animated animate__fadeInDown animate__slow"
>
    <button class="w-16 h-16 relative" onclick={addMusicAndPlay}>
        <div
            class="music-item-play bg-black bg-opacity-50 absolute grid w-full h-full
        justify-items-center items-center animate__animated animate__faster animate__fadeOut rounded"
        >
            <img
                class="w-12 h-12 invert"
                src={MusicConfig.defaultPlayButton}
                alt="Play"
            />
        </div>
        <img class="rounded" src={albumImage} alt="Album" />
    </button>
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-200">
            {MusicController.getFullArtistFromMusic(music)}
        </p>
    </div>
</div>

<style lang="scss">
    .music-item-play:hover {
        animation-name: fadeIn;
    }
</style>
