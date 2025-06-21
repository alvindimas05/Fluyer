<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";
import PlaylistItem from "./PlaylistItem.svelte";
import { SidebarType } from "$lib/home/sidebar/types";
import Icon from "$lib/icon/Icon.svelte";
import {IconType} from "$lib/icon/types";
import MusicController from "$lib/controllers/MusicController";

function cleanPlaylist(){
    MusicController.reset();
}
</script>

<Sidebar type={SidebarType.Right}>
    <div class="grid grid-cols-[auto_max-content] items-center p-3">
        <p class="text-[1.5rem] font-semibold">Playlist</p>
        <button class="w-7" onclick={cleanPlaylist}>
            <Icon type={IconType.CleanPlaylist} />
        </button>
    </div>
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