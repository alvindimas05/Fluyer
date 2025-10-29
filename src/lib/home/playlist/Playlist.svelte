<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import {
    musicCurrentIndex,
    musicPlaylist, musicPlaylistIds,
    musicReset,
} from "$lib/stores/music";
import PlaylistItem from "./PlaylistItem.svelte";
import { SidebarType } from "$lib/home/sidebar/types";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import MusicController from "$lib/controllers/MusicController";
import Muuri from "muuri";
import { mount, onDestroy, onMount, unmount } from "svelte";
import { isMobile } from "$lib/platform";
import type { MusicData } from "$lib/home/music/types";
import type { Unsubscriber } from "svelte/store";

let gridElement: HTMLDivElement;
let muuri: Muuri;
let dragging = $state(!isMobile());
let oldPlaylist: MusicData[] = [];
let fromIndex = $state(-1);

function cleanPlaylist() {
	MusicController.reset();
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
            const isCurrentlyPlaying = itemIndex === $musicCurrentIndex;
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
        if (!$musicPlaylist) return;
        fromIndex = muuri.getItems().indexOf(item);
    });

    // Handle reordering when drag ends
    muuri.on("dragEnd", async (item, _) => {
        if (!muuri || !$musicPlaylist) return;

        const toIndex = muuri.getItems().indexOf(item);

        // No action needed if item wasn't moved
        if (fromIndex === toIndex) return;

        dragging = false;

        let playlistIds = $musicPlaylistIds;
        const uuid = playlistIds[fromIndex];
        playlistIds.splice(fromIndex, 1);
        playlistIds.splice(toIndex, 0, uuid);
        $musicPlaylistIds = playlistIds;

        // Temporarily disable dragging during playlist update
        await MusicController.playlistMoveto(fromIndex, toIndex);
        dragging = true;
    });

    return muuri;
}

// Creates a DOM element for a playlist item
function createPlaylistItem(music: MusicData, uuid: string) {
    const element = document.createElement("div");
    element.className = "playlist-item absolute w-full h-fit";

    // Mount the Svelte component to the element
    mount(PlaylistItem, { target: element, props: { music, uuid } });

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

let unlistenMusicPlaylist: Unsubscriber;

onMount(() => {
    initMuuri();

    unlistenMusicPlaylist = musicPlaylist.subscribe((playlist) => {
        // Determine which items were removed from the playlist
        const removedIndices = $musicReset
            ? oldPlaylist.map((_, index) => index) // Reset: remove all items
            : oldPlaylist
                .map((music, index) =>
                    !playlist.includes(music) ? index : -1
                )
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
        const newItems = $musicReset
            ? playlist.map((music, index) => ({ music, index })) // Reset: all items are new
            : playlist
                .map((music, index) =>
                    !oldPlaylist.includes(music)
                        ? { music, index }
                        : null
                )
                .filter(
                    (item): item is { music: MusicData; index: number } =>
                        item !== null
                );

        // Add new items to Muuri grid
        if (newItems.length > 0) {
            let playlistIds: string[] = [];
            muuri.add(
                newItems.map(({ music }) => {
                    const uuid = crypto.randomUUID();
                    playlistIds.push(uuid);
                    return createPlaylistItem(music, uuid);
                })
            );
            $musicPlaylistIds = playlistIds;
        }

        // Update touch behavior for all playlist items
        elementToggleDraggable();

        // Store current playlist state for next comparison
        oldPlaylist = [...playlist];
    });
});

onDestroy(() => {
    // Clean up subscription when component is destroyed
    unlistenMusicPlaylist();
});

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