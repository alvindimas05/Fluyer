<script lang="ts">
import Icon from "$lib/ui/icon/Icon.svelte";
import { IconType } from "$lib/ui/icon/types";
import Muuri from "muuri";
import { mount, onMount, unmount } from "svelte";
import { isMobile } from "$lib/platform";
import Sidebar from "$lib/features/sidebar/components/Sidebar.svelte";
import type {MusicData} from "$lib/features/music/types";
import ProgressService from "$lib/services/ProgressService.svelte";
import MusicPlayerService from "$lib/services/MusicPlayerService.svelte";
import musicStore from "$lib/stores/music.svelte";
import {SidebarType} from "$lib/features/sidebar/types";
import QueueService from "$lib/services/QueueService.svelte";
import MusicQueueItem from "$lib/features/music_queue/components/MusicQueueItem.svelte";

let gridElement: HTMLDivElement;
let muuri: Muuri;
let dragging = $state(!isMobile());
let oldPlaylist: MusicData[] = [];
let fromIndex = $state(-1);

function cleanPlaylist() {
    MusicPlayerService.pause();
    ProgressService.reset();
}

function dragToggle() {
    dragging = !dragging;
}

// Initializes the Muuri grid library for drag-and-drop playlist management
function initMuuri() {
    muuri = new Muuri(gridElement, {
        // Enable drag and drop functionality
        dragEnabled: true,

        // Configure drag behavior to move items rather than swap
        dragSortPredicate: {
            action: "move",
        },

        // Determine whether an item can be dragged
        dragStartPredicate: (item, e) => {
            const itemIndex = muuri.getItems().indexOf(item);
            const isCurrentlyPlaying = itemIndex === musicStore.currentIndex;
            const isDraggableElement = e.target.classList.contains("muuri-draggable");

            // Prevent dragging if:
            // - Item is currently playing
            // - Click target isn't a draggable element
            // - Global dragging is disabled
            return !(isCurrentlyPlaying || !isDraggableElement || !dragging);
        },
    });

    // Store the starting position when drag begins
    muuri.on("dragStart", (item, _) => {
        if (!musicStore.list) return;
        fromIndex = muuri.getItems().indexOf(item);
    });

    // Handle reordering when drag ends
    muuri.on("dragEnd", async (item, _) => {
        if (!muuri || !musicStore.list) return;

        const toIndex = muuri.getItems().indexOf(item);

        // No action needed if item wasn't moved
        if (fromIndex === toIndex) return;

        dragging = false;

        let playlistIds = musicStore.listIds;
        const uuid = playlistIds[fromIndex];
        playlistIds.splice(fromIndex, 1);
        playlistIds.splice(toIndex, 0, uuid);
        musicStore.listIds = playlistIds;

        // Temporarily disable dragging during playlist update
        await QueueService.moveTo(fromIndex, toIndex);
        dragging = true;
    });

    return muuri;
}

// Creates a DOM element for a playlist item
function createPlaylistItem(music: MusicData, uuid: string) {
    const element = document.createElement("div");
    element.className = "playlist-item absolute w-full h-fit";

    // Mount the Svelte component to the element
    mount(MusicQueueItem, { target: element, props: { music, uuid } });

    return element;
}

// Toggles touch-action CSS property on playlist items based on dragging state
// This enables/disables native touch scrolling during drag operations
function elementToggleDraggable() {
    const elements = document.querySelectorAll(".playlist-item");

    if (dragging) {
        // Disable native touch actions to allow dragging
        elements.forEach((el) => {
            (el as HTMLDivElement).style.touchAction = "none";
        });
    } else {
        // Re-enable native touch actions (e.g., scrolling)
        elements.forEach((el) => {
            (el as HTMLDivElement).style.touchAction = "auto";
        });
    }
}

$effect(() => {
    const playlist = musicStore.list!!;
    // Determine which items were removed from the playlist
    const removedIndices = musicStore.reset
        ? oldPlaylist.map((_, index) => index) // Reset: remove all items
        : oldPlaylist
            .map((music, index) => (!playlist.includes(music) ? index : -1))
            .filter((index) => index !== -1); // Keep only valid indices

    // Remove items from Muuri grid
    if (removedIndices.length > 0) {
        const items = muuri.getItems();
        const removedItems = removedIndices.map((i) => items[i]);

        // Unmount Svelte components before removing
        for (const item of removedItems) {
            unmount(item.getElement()!!);
        }

        muuri.remove(removedItems, { removeElements: true });
    }

    // Determine which items are new to the playlist
    const newItems = musicStore.reset
        ? playlist.map((music, index) => ({ music, index })) // Reset: all items are new
        : playlist
            .map((music, index) =>
                !oldPlaylist.includes(music) ? { music, index } : null,
            )
            .filter(
                (item): item is { music: MusicData; index: number } =>
                    item !== null,
            );

    // Add new items to Muuri grid
    if (newItems.length > 0) {
        let playlistIds = musicStore.reset ? [] : musicStore.listIds;
        muuri.add(
            newItems.map(({ music }) => {
                const uuid = crypto.randomUUID();
                playlistIds.push(uuid);
                return createPlaylistItem(music, uuid);
            }),
        );
        musicStore.listIds = playlistIds;
    }

    // Update touch behavior for all playlist items
    elementToggleDraggable();

    // Store current playlist state for next comparison
    oldPlaylist = playlist;
})

onMount(initMuuri);

// Reactively update draggable state when dependencies change
$effect(elementToggleDraggable);
</script>

<Sidebar type={SidebarType.Right} class="flex flex-col">
    <div class="grid grid-cols-[auto_max-content_max-content] items-center gap-3 p-3">
        <p class="text-[1.2rem] md:text-[1.5rem] font-semibold">Now Playing</p>
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
    <div class="flex-1 w-full overflow-auto scrollbar-hidden">
        <div class="w-full relative" bind:this={gridElement}></div>
    </div>
</Sidebar>