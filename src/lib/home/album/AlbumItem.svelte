<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "../music/types";
import MusicController from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import Icon from "$lib/icon/Icon.svelte";
import {IconType} from "$lib/icon/types";

interface Props {
	musicList: MusicData[];
	index: number;
}

let { musicList, index }: Props = $props();
let music = MusicController.sortMusicList(musicList)[0];

const animationDelay = 200;
let animationClasses = $state("hidden");

// const spotifyApi = new SpotifyApi();
let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null || music.artist == null || music.album == null)
		return;
	const status = await CoverArt.fromQuery({
		artist: music.artist!,
		album: music.album!,
	});
	if (status == CoverArtStatus.Failed) return;
	if (status == CoverArtStatus.Loading) {
		const unlisten = coverArtCaches.subscribe(() => {
			if (setAlbumImageFromCache()) setTimeout(() => unlisten(), 0);
		});
		return;
	}

	setAlbumImageFromCache();
}

function setAlbumImageFromCache() {
	const cache = MusicController.getCoverArtCache({
		artist: music.artist!,
		album: music.album!,
	});
	if (
		cache == null ||
		(cache.status == CoverArtStatus.Loading && cache.image == null)
	)
		return false;
	if (cache.status == CoverArtStatus.Failed) return true;

	albumImage = MusicController.withBase64(cache.image!);
	musicList = musicList.map((m) => {
		m.image = MusicController.withBase64(cache.image!);
		return m;
	});
	return true;
}

async function addMusicListAndPlay() {
	music.image = albumImage;
	await MusicController.reset();
	await MusicController.addMusicList(MusicController.sortMusicList(musicList));
	MusicController.play();
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
                class="w-12 md:w-14
                 absolute bottom-0 left-0 ms-3 mb-3"
                onclick={addMusicListAndPlay}
            ><Icon type={IconType.Play}/></button>
        </div>
        <img class="rounded-lg w-full" src={albumImage} alt="Album" />
    </div>
    <p class="font-medium sm:text-lg md:text-xl mt-2 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">{music.album}</p>
    <p class="text-[15px] sm:text-base md:text-lg text-opacity-background-80 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {MusicController.getFullArtistFromMusic(music)}
    </p>
</div>

<style lang="scss">
    .album-item-actions:hover {
        animation-name: fadeIn;
    }
</style>
