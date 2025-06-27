<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import { musicPlaylist } from "$lib/stores/music";
import PlaylistItem from "./PlaylistItem.svelte";
import { SidebarType } from "$lib/home/sidebar/types";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import MusicController from "$lib/controllers/MusicController";
import Muuri from "muuri";
import { mount, onMount, unmount } from "svelte";
import { isMobile } from "$lib/platform";
import type { MusicData } from "$lib/home/music/types";

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

function initMuuri() {
	muuri = new Muuri(gridElement, {
		dragEnabled: true,
		dragSortPredicate: {
			action: "move",
		},
		dragStartPredicate: () => dragging,
	});

	muuri.on("dragStart", (item, _) => {
		if (!$musicPlaylist) return;
		fromIndex = muuri.getItems().indexOf(item);
	});

	muuri.on("dragEnd", (item, _) => {
		if (!muuri || !$musicPlaylist) return;
		const toIndex = muuri.getItems().indexOf(item);

		if (fromIndex === toIndex) return;

		MusicController.playlistMoveto(fromIndex, toIndex);
	});

	return muuri;
}

function createPlaylistItem(music: MusicData) {
	const element = document.createElement("div");
	element.className = "absolute w-full h-fit";
	mount(PlaylistItem, { target: element, props: { music } });
	return element;
}

onMount(() => {
	initMuuri();

	musicPlaylist.subscribe((playlist) => {
		if (playlist.length < 1) return;

		const removedIndices = oldPlaylist
			.map((music, index) => (!playlist.includes(music) ? index : -1))
			.filter((index) => index !== -1);

		if (removedIndices.length > 0) {
			const items = muuri.getItems();
			const removedItems = removedIndices.map((i) => items[i]);
			for (const item of removedItems) {
				unmount(item.getElement()!!);
			}
			muuri.remove(removedItems, { removeElements: true });
		}

		const newItems = playlist
			.map((music, index) =>
				!oldPlaylist.includes(music) ? { music, index } : null,
			)
			.filter(
				(item): item is { music: MusicData; index: number } => item !== null,
			);

		if (newItems.length > 0) {
			muuri.add(newItems.map(({ music }) => createPlaylistItem(music)));
		}

		oldPlaylist = [...playlist];
	});
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
    <div class="flex-1 w-full overflow-auto scrollbar-hidden">
        <div class="w-full relative" bind:this={gridElement}></div>
    </div>
</Sidebar>