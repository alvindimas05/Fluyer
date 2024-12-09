<script>
    import { musicList } from "$lib/stores/music";
    import PlaylistItem from "./PlaylistItem.svelte";
</script>

<div
    class="fixed right-0 top-0 z-10 bg-gray-700 bg-opacity-30 backdrop-blur-md rounded
    text-white me-3 mt-8 h-[calc(100%-8rem)] w-[25%] p-3"
>
    <div class="border border-gray-400 h-full w-full rounded flex flex-col">
        <p class="text-[1.5rem] font-semibold p-3">Playlist</p>
        <div class="flex-1 w-full overflow-auto scrollbar-hidden">
            {#each $musicList as music, index}
                <PlaylistItem {music} isPlaying={index == 6} />
            {/each}
        </div>
    </div>
</div>
