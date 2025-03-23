<script lang="ts">
import type { MusicData } from "../music/types";

interface Props {
	music: MusicData;
	index: number;
	isPlaying?: boolean;
	isPrevious?: boolean;
}
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";

let {
	music,
	index = -1,
	isPlaying = false,
	isPrevious = false,
}: Props = $props();

function removePlaylist() {
	MusicController.removeMusic(index);
}

function gotoPlaylist() {
	MusicController.gotoPlaylist(index);
}
</script>

<div
    class={`grid grid-cols-[max-content_auto_max-content] py-2 px-3 ${isPlaying && "bg-gray-700 bg-opacity-40"}
    animate__animated animate__fadeIn`}
>
    <button
        class={`w-12 h-12 lg:w-16 lg:h-16 relative ${isPlaying && "cursor-defaultj"}`}
        onclick={() => !isPlaying && gotoPlaylist()}
    >
        {#if !isPlaying}
            <div
                class="playlist-item-goto bg-black bg-opacity-40 absolute grid w-full h-full z-10
            justify-items-center items-center animate__animated animate__faster animate__fadeOut rounded"
            >
                <img
                    class="w-10 h-10 lg:w-12 lg:h-12 invert"
                    src={isPrevious
                        ? MusicConfig.defaultPreviousButton
                        : MusicConfig.defaultNextButton}
                    alt="Go To"
                />
            </div>
        {/if}
        <div class="w-12 h-12 lg:w-16 lg:h-16 relative">
            <img
                class="w-12 lg:w-16 rounded"
                src={MusicController.getAlbumImageFromMusic(music)}
                alt="Album"
            />
        </div></button
    >

    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-opacity-background-70">
            {MusicController.getFullArtistFromMusic(music)}
        </p>
    </div>
    <button
        class={`w-12 lg:w-16 flex justify-center items-center animate__animated
        ${isPlaying ? "animate__pulse animate__infinite cursor-default" : "animate__faster animate__fadeOut playlist-item-remove"}`}
        onclick={() => !isPlaying && removePlaylist()}
    >
        <img
            class={`w-10 h-10 invert`}
            src={isPlaying
                ? MusicConfig.defaultPlayingIcon
                : MusicConfig.defaultPlaylistRemoveButton}
            alt={isPlaying ? "Playing" : "Remove"}
        />
    </button>
</div>

<style lang="scss">
    .playlist-item-remove,
    .playlist-item-goto {
        &:hover {
            animation-name: fadeIn;
        }
    }
</style>
