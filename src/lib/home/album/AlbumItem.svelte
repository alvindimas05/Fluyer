<script lang="ts">
import { onMount } from "svelte";
import type { AlbumData, MusicData } from "../music/types";
import MusicController, {MusicConfig} from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import { filterAlbum, filterSearch } from "$lib/stores/filter";
import FilterController from "$lib/controllers/FilterController";
import { musicList as storeMusicList } from "$lib/stores/music";

interface Props {
	musicList: MusicData[];
}

let { musicList }: Props = $props();
let music = $derived(musicList[0]);

let isValidFilterAlbum = $derived(
	$filterAlbum && music.album && $filterAlbum.name === music.album,
);

let albumImage = $derived(MusicController.getAlbumImageFromMusic(music));

async function setFilterAlbum() {
	FilterController.setFilterAlbum({
		name: music.album,
		artist: music.albumArtist ?? MusicController.getFullArtistFromMusic(music),
		musicList,
	} as AlbumData);
}

// onMount(checkAlbumImage);
</script>

<div
    class="px-3 pb-3 text-white row-[1] col-auto animate__animated animate__fadeIn"
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
        {#await albumImage}
            <img class="rounded-lg w-full shadow-lg"
                 src={MusicConfig.defaultAlbumImage}
                 alt="Album" />
        {:then image}
            <img class="rounded-lg w-full shadow-lg"
                 src={image}
                 alt="Album" />
        {/await}
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
