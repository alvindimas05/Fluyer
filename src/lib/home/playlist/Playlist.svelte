<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";
import PlaylistItem from "./PlaylistItem.svelte";
import { SidebarType } from "$lib/home/sidebar/types";
</script>

<Sidebar type={SidebarType.Right}>
    <p class="text-[1.5rem] font-semibold p-3">Playlist</p>
    <div class="flex-1 w-full overflow-auto scrollbar-hidden">
        {#each $musicPlaylist as music, index}
            <PlaylistItem
                {music}
                {index}
                isPlaying={$musicCurrentIndex === index}
                isPrevious={index < $musicCurrentIndex}
            />
        {/each}
    </div>
</Sidebar>