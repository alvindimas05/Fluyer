<script lang="ts">
import {type MusicData} from "$lib/features/music/types";
import {isDesktop, isLinux} from "$lib/platform";
import {useAlbumItem} from "$lib/features/album/viewmodels/useAlbumItem.svelte";

interface Props {
    musicList: MusicData[];
    index: number;
}

let { musicList, index }: Props = $props();

const vm = useAlbumItem(musicList, index);
</script>

<div
    class="h-fit px-3 pb-3 row-[1] col-auto"
>
    <div class="relative w-full">
        {#if vm.isValidFilterAlbum}
            <div class="w-full h-full absolute top-0 left-0 z-10
            rounded-lg shadow-[inset_0_0_0_2px_white]"></div>
        {:else}
            <div class="album-item-actions w-full h-full absolute rounded-lg z-20
                bg-white/20 shadow-[inset_0_0_0_2px_white] cursor-pointer"
                 onclick={vm.setFilterAlbum}
            ></div>
        {/if}
        {#await vm.albumImage}
            <div class="w-full aspect-square"></div>
        {:then image}
            <img class="rounded-lg w-full aspect-square object-cover {isDesktop() && !isLinux() && 'animate__animated animate__fadeIn'}"
                 src={image}
                 alt="Album" />
        {/await}
    </div>
    <p class="font-medium md:text-lg mt-2 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {vm.music.album}
    </p>
    <p class="text-[15px] md:text-base text-opacity-background-80 whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
        {vm.music.albumArtist ?? vm.music.artist}
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
