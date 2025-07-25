<script lang="ts">
import type { MusicData } from "../music/types";

interface Props {
	music: MusicData;
}
import MusicController from "$lib/controllers/MusicController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";

let { music }: Props = $props();

let index = $derived($musicPlaylist.findIndex((m) => m.path === music.path));
let isPlaying = $derived($musicCurrentIndex === index);
let isPrevious = $derived(index < $musicCurrentIndex);

function removePlaylist() {
	MusicController.removeMusic(index);
}

function gotoPlaylist() {
	MusicController.gotoPlaylist(index);
}
</script>

<div
    class={`relative grid grid-cols-[max-content_auto_max-content] py-2 px-3 animate__animated animate__fadeIn
        ${isPlaying && "bg-gray-700 bg-opacity-40"}`}
>
    <div class="w-11 md:w-12 lg:w-14">
        <img
            class="w-full aspect-square rounded"
            src={MusicController.getAlbumImageFromMusic(music)}
            alt="Album"
        />
    </div>
    <div class="ms-3 text-sm md:text-base">
        <p class="font-medium">{music.title}</p>
        <p class="text-opacity-background-70">
            {MusicController.getFullArtistFromMusic(music)}
        </p>
    </div>
    <div class="w-11 md:w-12 lg:w-14"></div>
</div>
{#if isPlaying}
    <div
        class="absolute top-0 left-0 w-full grid grid-cols-[max-content_auto_max-content] py-2 px-3 z-10"
    >
        <div class="w-11 h-11 md:w-12 md:h-12 lg:w-14 lg:h-14 aspect-square"></div>
        <div></div>
        <div class="w-11 md:w-12 lg:w-14 aspect-square p-1 lg:p-3">
            <div class="w-full animate__animated animate__infinite animate__pulse">
                <Icon type={IconType.Playing} />
            </div>
        </div>
    </div>
{:else}
    <div
        class="absolute top-0 left-0 w-full grid grid-cols-[max-content_auto_max-content] py-2 px-3 z-10
        animate__animated animate__faster animate__fadeOut playlist-item-controls"
    >
        <button
            class="w-11 h-11 md:w-12 md:h-12 lg:w-14 lg:h-14 aspect-square"
            onclick={gotoPlaylist}
        >
            {#if !isPlaying}
                <div
                    class="w-full h-full bg-black bg-opacity-40 rounded lg:p-1"
                >
                    <Icon type={isPrevious ? IconType.Previous : IconType.Next} />
                </div>
            {/if}
        </button>
        <div class="muuri-draggable"></div>
        <button class="w-11 md:w-12 lg:w-14 aspect-square lg:p-1"
            onclick={removePlaylist}>
            <Icon type={IconType.Remove}/>
        </button>
    </div>
{/if}

<style lang="scss">
    .playlist-item-controls:hover {
        animation-name: fadeIn;
    }
</style>
