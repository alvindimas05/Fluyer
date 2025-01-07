<script lang="ts">
    import type { MusicData } from "../music/types";

    interface Props {
        music: MusicData;
        index: number;
        isPlaying?: boolean;
    }
    import MusicController, {
        MusicConfig,
    } from "$lib/controllers/MusicController";

    let { music, index, isPlaying = false }: Props = $props();

    function removePlaylist() {
        if(index === 0) MusicController.nextMusic();
        else MusicController.removeMusic(index - 1);
    }
</script>

<div
    class={`grid grid-cols-[max-content_auto_max-content] py-2 px-3 ${isPlaying ? "bg-gray-700 bg-opacity-40" : ""}
    animate__animated animate__fadeIn`}
>
    <button class="w-12 h-12 lg:w-16 lg:h-16 relative" onclick={removePlaylist}>
        <div
            class="playlist-item-remove bg-black bg-opacity-40 absolute grid w-full h-full
        justify-items-center items-center animate__animated animate__faster animate__fadeOut rounded"
        >
            <img
                class="w-10 h-10 lg:w-12 lg:h-12 invert"
                src={MusicConfig.defaultPlaylistRemoveButton}
                alt="Play"
            />
        </div>
        <img
            class="w-12 lg:w-16 rounded"
            src={MusicController.getAlbumImageFromMusic(music)}
            alt="Album"
        />
    </button>
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-200">
            {MusicController.getFullArtistFromMusic(music)}
        </p>
    </div>
    {#if isPlaying}
        <div class="w-12 lg:w-16 flex justify-center items-center">
            <img
                class="w-10 h-10 invert animate__animated animate__pulse animate__infinite"
                src={MusicConfig.defaultPlayingIcon}
                alt="Playing"
            />
        </div>
    {/if}
</div>

<style lang="scss">
    .playlist-item-remove:hover {
        animation-name: fadeIn;
    }
</style>
