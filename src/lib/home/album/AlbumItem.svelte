<script lang="ts">
import SpotifyApi from "$lib/api/spotify";
import { onMount } from "svelte";
import type { MusicData } from "../music/types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import MusicBrainzApi from "$lib/api/musicbrainz";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import { isAndroid, isIos, isMacos } from "$lib/platform";

interface Props {
	musicList: MusicData[];
	index: number;
}

let { musicList, index }: Props = $props();
let music = musicList[0];

const animationDelay = 200;
let animationClasses = $state("hidden");
let isSorted = false;

// const spotifyApi = new SpotifyApi();
let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null || music.artist == null || music.album == null)
		return;
	// const spotifyMusic = await spotifyApi.searchMusic(music);
	// if (spotifyMusic == null) return;
	// albumImage = spotifyMusic?.imageUrl;
	const status = await CoverArt.fromQuery({
		artist: music.artist!,
		album: music.album!,
	});
	if (status == CoverArtStatus.Failed) return;
	if (status == CoverArtStatus.Loading) {
		// Note: Blame Webkit for this shit. Always gives error "Uninitialized variable" when trying to call unlisten :)
		// Note: I have no idea why this happens on Android as well.
		if (isMacos() || isIos() || isAndroid()) {
			coverArtCaches.subscribe(() => {
				setAlbumImageFromCache();
			});
		} else {
			const unsub = coverArtCaches.subscribe(() => {
				if (setAlbumImageFromCache()) unsub();
			});
		}
		return;
	}

	setAlbumImageFromCache();
}

function setAlbumImageFromCache() {
	const cache = MusicController.getCoverArtCache({
		artist: music.artist!,
		album: music.album!,
	});
	if (cache == null) return false;
	if (cache.status == CoverArtStatus.Failed || cache.image == null) return true;

	albumImage = MusicController.withBase64(cache.image!);
	musicList = musicList.map((m) => {
		m.image = MusicController.withBase64(cache.image!);
		return m;
	});
	return true;
}

async function addMusicListAndPlay() {
	music.image = albumImage;
	await MusicController.clear();
	await MusicController.addMusicList(MusicController.sortMusicList(musicList));
	MusicController.play(true);
}

onMount(() => {
    checkAlbumImage();
});

setTimeout(
	() => (animationClasses = "animate__animated animate__fadeInDown"),
	animationDelay * index,
);
</script>

<div
    class={`px-3 pb-6 text-white row-[1] col-auto ${animationClasses}`}
>
    <div class="relative w-full">
        <div
            class="album-item-actions w-full h-full absolute rounded-lg bg-gradient-to-b from-transparent to-black/50
			animate__animated animate__faster animate__fadeOut"
        >
            <button
                class="w-12 h-12 absolute bottom-0 left-0 ms-3 mb-3"
                onclick={addMusicListAndPlay}
            >
                <img
                    class="invert"
                    src={MusicConfig.defaultPlayButton}
                    alt="Play"
                /></button
            >
        </div>
        <img class="rounded-lg w-full" src={albumImage} alt="Album" />
    </div>
    <p class="font-medium text-xl mt-2 whitespace-nowrap overflow-hidden">{music.album}</p>
    <p class="text-lg text-gray-200 whitespace-nowrap overflow-hidden">
        {MusicController.getFullArtistFromMusic(music)}
    </p>
</div>

<style lang="scss">
    .album-item-actions:hover {
        animation-name: fadeIn;
    }
</style>
