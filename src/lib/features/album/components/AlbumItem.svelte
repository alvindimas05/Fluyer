<script lang="ts">
import {type AlbumData, type MusicData, MusicListType} from "$lib/features/music/types";
import {isDesktop, isLinux} from "$lib/platform";
import MetadataService from "$lib/services/MetadataService.svelte";
import filterStore from "$lib/stores/filter.svelte";
import musicStore from "$lib/stores/music.svelte";
import ProgressService from "$lib/services/ProgressService.svelte";

interface Props {
    musicList: MusicData[];
    index: number;
}

let { musicList, index }: Props = $props();
let music = $derived(musicList[0]);

let isValidFilterAlbum = $derived(
    filterStore.album && music.album && filterStore.album.name === music.album,
);

let albumImage = $derived.by(async () => MetadataService.getMusicCoverArt(music));

async function setFilterAlbum() {
    const isAlbumType = musicStore.listType === MusicListType.Album;
    musicStore.listType = MusicListType.All;
    filterStore.album = {
        name: music.album,
        artist: music.albumArtist ?? music.artist,
        year: MetadataService.getYearFromDate(music.date),
        duration: ProgressService.formatDuration(
            musicList.map((m) => m.duration).reduce((a, b) => a + b, 0),
        ),
        musicList,
    } as AlbumData;
    if(isAlbumType) setTimeout(() => musicStore.albumListScrollIndex = index, 500);
}
</script>

<div
    class="h-fit px-3 pb-3 row-[1] col-auto"
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
            <img class="rounded-lg w-full aspect-square object-cover {isDesktop() && !isLinux() && 'animate__animated animate__fadeIn'}"
                 src={image}
                 alt="Album" />
        {/await}
    </div>
    <p class="font-medium md:text-lg mt-2 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {music.album}
    </p>
    <p class="text-[15px] md:text-base text-opacity-background-80 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {music.albumArtist ?? music.artist}
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
