<script lang="ts">
import { onMount } from "svelte";
import type { AlbumData, MusicData } from "../music/types";
import MusicController, {MusicConfig} from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import { filterAlbum, filterSearch } from "$lib/stores/filter";
import FilterController from "$lib/controllers/FilterController";
import { musicList as storeMusicList } from "$lib/stores/music";
import {isDesktop} from "$lib/platform";

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
        year: MusicController.getYearFromDate(music.date),
        duration: MusicController.parseMilisecondsIntoText(musicList.map(m => m.duration).reduce((a, b) => a + b, 0)),
		musicList,
	} as AlbumData);
}
</script>

<div
    class="h-fit px-3 pb-3 text-white row-[1] col-auto"
>
    <div class="relative w-full">
        {#if isValidFilterAlbum}
            <div class="w-full h-full absolute top-0 left-0 z-10
            rounded-lg shadow-[inset_0_0_0_2px_white]"></div>
        {:else}
            <div class="album-item-actions w-full h-full absolute rounded-lg z-20
                bg-white/20 shadow-[inset_0_0_0_2px_white] cursor-pointer"
                    onclick={setFilterAlbum}
            ></div>
        {/if}
        {#await albumImage}
            <div class="w-full aspect-square"></div>
        {:then image}
            <img class="rounded-lg w-full {isDesktop() && 'animate__animated animate__fadeIn'}"
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
    .album-item-actions {
        opacity: 0;

        &:hover {
            animation-name: fadeIn;
            animation-duration: 0.5s;
            animation-fill-mode: forwards;
        }
    }
</style>
