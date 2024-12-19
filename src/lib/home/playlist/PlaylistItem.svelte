<script lang="ts">
import type { MusicData } from "../music/types";

interface Props {
	music: MusicData;
	isPlaying?: boolean;
}
import MusicController from "$lib/controllers/MusicController";

let { music, isPlaying = false }: Props = $props();
</script>

<div
    class={`grid grid-cols-[max-content_auto_max-content] py-2 px-3 ${isPlaying ? "bg-gray-700 bg-opacity-40" : ""}
    animate__animated animate__fadeIn`}
>
    <button class="relative"
        ><img
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
                src={"/icons/default/playing.png"}
                alt="Playing"
            />
        </div>
    {/if}
</div>
