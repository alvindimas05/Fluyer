<script lang="ts">
import { onMount } from "svelte";
import type { AlbumData, MusicData } from "../music/types";
import MusicController from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import { filterAlbum, filterSearch } from "$lib/stores/filter";
import FilterController from "$lib/controllers/FilterController";
import {musicList as storeMusicList} from "$lib/stores/music";

interface Props {
	musicList: MusicData[];
	index: number;
}

let { musicList, index }: Props = $props();
let music = MusicController.sortMusicList(musicList)[0];

let isValidSearch = $derived.by(() => {
	const search = $filterSearch.toLowerCase();
	const hasSearch = !!search;
	const hasFilterAlbum = !!$filterAlbum;

	if (hasFilterAlbum) return true;

	if (!hasSearch) return true;

	return (
		music.album?.toLowerCase().includes(search) ||
		music.albumArtist?.toLowerCase().includes(search)
	);
});

let isValidFilterAlbum = $derived(
	$filterAlbum && music.album && $filterAlbum.name === music.album,
);

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
	storeMusicList.update((list) => {
        if(!Array.isArray(list)) return list;
        for (let i = 0; i < list.length; i++) {
            if (list[i].album == music.album && list[i].artist == music.artist) {
                list[i].image = MusicController.withBase64(cache.image!);
            }
        }
        return list;
    });
	return true;
}

async function setFilterAlbum() {
	FilterController.setFilterAlbum({
		name: music.album,
		artist: music.albumArtist ?? MusicController.getFullArtistFromMusic(music),
		musicList,
	} as AlbumData);
}

onMount(() => {
	checkAlbumImage();
});

if(index < 10){
    setTimeout(
        () => (animationClasses = "animate__animated animate__fadeIn"),
        animationDelay * index,
    );
}
</script>

<div
    class={`px-3 pb-3 text-white row-[1] col-auto ${animationClasses} ${!isValidSearch && "hidden"}`}
>
    <div class="relative w-full">
        <div
            class="album-item-actions w-full h-full absolute rounded-lg z-20
            bg-white/20 shadow-[inset_0_0_0_2px_white] cursor-pointer
			animate__animated animate__faster animate__fadeOut"
            onclick={setFilterAlbum}
        ></div>
        {#if isValidFilterAlbum}
            <div class="w-full h-full absolute top-0 left-0 z-10
            rounded-lg shadow-[inset_0_0_0_2px_white]"></div>
        {/if}
        <img class={`rounded-lg w-full shadow-lg`}
             src={albumImage}
             alt="Album" />
    </div>
    <p class="font-medium md:text-lg mt-2 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {music.album}
    </p>
    <p class="text-[15px] md:text-base text-opacity-background-80 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {music.albumArtist ?? MusicController.getFullArtistFromMusic(music)}
    </p>
</div>

<style lang="scss">
    .album-item-actions:hover {
        animation-name: fadeIn;
    }
</style>
