<script lang="ts">
    import SpotifyApi from "$lib/api/spotify";
    import { onMount } from "svelte";
    import type { MusicData } from "../music/types";

    interface Props {
        music: MusicData;
        index: number;
    }

    let { music, index }: Props = $props();

    const animationDelay = 200;
    let animationClasses = $state("hidden");

    const spotifyApi = new SpotifyApi();
    let albumImage = $state(
        music.image
            ? `data:image/png;base64,${music.image}`
            : "/icons/default/default-album-cover.jpg",
    );

    async function checkAlbumImage() {
        if (music.image !== null) return;
        const spotifyMusic = await spotifyApi.searchMusic(music);
        albumImage = spotifyMusic?.imageUrl;
    }

    onMount(checkAlbumImage);

    setTimeout(
        () =>
            (animationClasses =
                "animate__animated animate__fadeInDown"),
        animationDelay * index
    );
</script>

<div class={`px-3 py-6 text-white row-[1] col-auto ${animationClasses}`}>
    <img class="rounded-lg" src={albumImage} alt="Album" />
    <p class="font-medium text-xl mt-2">{music.album}</p>
    <p class="text-lg text-gray-200">{music.album_artist ?? music.artist}</p>
</div>
