<script lang="ts">
    import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
    import { musicCurrentIndex, musicPlaylist } from "$lib/stores/music";
    import PlaylistItem from "./PlaylistItem.svelte";
    import { SidebarType } from "$lib/home/sidebar/types";
    import Icon from "$lib/icon/Icon.svelte";
    import { IconType } from "$lib/icon/types";
    import MusicController from "$lib/controllers/MusicController";
    import Muuri from "muuri";
    import { onMount } from "svelte";
    import {isMobile} from "$lib/platform";

    let gridElement: HTMLDivElement;
    let muuri: Muuri;
    let dragging = $state(!isMobile());

    function cleanPlaylist() {
        MusicController.reset();
    }

    function dragToggle() {
        dragging = !dragging;
    }

    function initMuuri() {
        if (muuri) muuri.destroy();
        muuri = new Muuri(gridElement, {
            dragEnabled: dragging,
            dragSortPredicate: {
                action: 'move'
            },
        });

        muuri.on('dragEnd', (item, _) => {
            if (!muuri || !$musicPlaylist) return;
            const items = muuri.getItems();
            const element = item.getElement()!!;
            const fromIndex = parseInt(element.getAttribute('data-index') || '0');
            const toIndex = items.indexOf(item);

            if(fromIndex === toIndex) return;

            MusicController.playlistMoveto(fromIndex, toIndex);
        });

        return muuri;
    }

    onMount(() => {
        initMuuri();

        // FIXME: Dirty method, might fix later
        musicPlaylist.subscribe(() => setTimeout(initMuuri));
    });
    $effect(() => {
        dragging;
        initMuuri();
    });
</script>

<Sidebar type={SidebarType.Right}>
    <div class="grid grid-cols-[auto_max-content_max-content] items-center gap-3 p-3">
        <p class="text-[1.5rem] font-semibold">Playlist</p>
        <button class="w-7" onclick={cleanPlaylist}>
            <Icon type={IconType.CleanPlaylist} />
        </button>
        {#if isMobile()}
            <button class="w-7" onclick={dragToggle}>
                {#if dragging}
                    <Icon type={IconType.DragOff} />
                {:else}
                    <Icon type={IconType.DragOn} />
                {/if}
            </button>
        {/if}
    </div>
    <div class="flex-1 w-full overflow-auto">
        <div class="w-full relative" bind:this={gridElement}>
            {#each $musicPlaylist as music, index}
                <PlaylistItem
                    {music}
                    {index}
                    isPlaying={$musicCurrentIndex === index}
                    isPrevious={index < $musicCurrentIndex}
                />
            {/each}
        </div>
    </div>
</Sidebar>