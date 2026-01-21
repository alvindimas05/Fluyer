<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import Muuri from 'muuri';
	import { mount, onMount, unmount } from 'svelte';
	import { isMobile } from '$lib/platform';
	import Sidebar from '$lib/features/sidebar/components/Sidebar.svelte';
	import type { MusicData } from '$lib/features/music/types';
	import ProgressService from '$lib/services/ProgressService.svelte';
	import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import QueueService from '$lib/services/QueueService.svelte';
	import MusicQueueItem from '$lib/features/music_queue/components/MusicQueueItem.svelte';

	let gridElement: HTMLDivElement;
	let muuri: Muuri;
	let dragging = $state(!isMobile());
	let oldQueueIds: string[] = [];
	let fromIndex = $state(-1);

	function cleanQueue() {
		MusicPlayerService.pause();
		ProgressService.reset();
	}

	function dragToggle() {
		dragging = !dragging;
	}

	// Initializes the Muuri grid library for drag-and-drop queue management
	function initMuuri() {
		muuri = new Muuri(gridElement, {
			// Enable drag and drop functionality
			dragEnabled: true,

			// Configure drag behavior to move items rather than swap
			dragSortPredicate: {
				action: 'move'
			},

			// Determine whether an item can be dragged
			dragStartPredicate: (item, e) => {
				const itemIndex = muuri.getItems().indexOf(item);
				const isCurrentlyPlaying = itemIndex === musicStore.currentIndex;
				const isDraggableElement = e.target.classList.contains('muuri-draggable');

				// Prevent dragging if:
				// - Item is currently playing
				// - Click target isn't a draggable element
				// - Global dragging is disabled
				return !(isCurrentlyPlaying || !isDraggableElement || !dragging);
			}
		});

		// Store the starting position when drag begins
		muuri.on('dragStart', (item, _) => {
			if (!musicStore.list) return;
			fromIndex = muuri.getItems().indexOf(item);
		});

		// Handle reordering when drag ends
		muuri.on('dragEnd', async (item, _) => {
			if (!muuri || !musicStore.list) return;

			const toIndex = muuri.getItems().indexOf(item);

			// No action needed if item wasn't moved
			if (fromIndex === toIndex) return;

			dragging = false;

			// Update queueIds to match new order
			let queueIds = [...musicStore.queueIds];
			const uuid = queueIds[fromIndex];
			queueIds.splice(fromIndex, 1);
			queueIds.splice(toIndex, 0, uuid);
			musicStore.queueIds = queueIds;

			// Temporarily disable dragging during queue update
			await QueueService.moveTo(fromIndex, toIndex);
			dragging = true;
		});
	}

	// Creates a DOM element for a queue item
	function createItem(music: MusicData, uuid: string) {
		const element = document.createElement('div');
		element.className = 'queue-item absolute w-full h-fit';

		// Mount the Svelte component to the element
		mount(MusicQueueItem, { target: element, props: { music, uuid } });

		return element;
	}

	// Toggles touch-action CSS property on queue items based on dragging state
	// This enables/disables native touch scrolling during drag operations
	function elementToggleDraggable() {
		const elements = document.querySelectorAll('.queue-item');

		if (dragging) {
			// Disable native touch actions to allow dragging
			elements.forEach((el) => {
				(el as HTMLDivElement).style.touchAction = 'none';
			});
		} else {
			// Re-enable native touch actions (e.g., scrolling)
			elements.forEach((el) => {
				(el as HTMLDivElement).style.touchAction = 'auto';
			});
		}
	}

	function refreshGrid() {
		const queue = musicStore.queue;
		const queueIds = musicStore.queueIds;

		if (!queue || queue.length === 0) {
			// Clear all items if queue is empty
			const items = muuri.getItems();
			if (items.length > 0) {
				for (const item of items) {
					unmount(item.getElement()!!);
				}
				muuri.remove(items, { removeElements: true });
			}
			oldQueueIds = [];
			return;
		}

		// Find removed IDs (in old but not in new)
		const newIdSet = new Set(queueIds);
		const oldIdSet = new Set(oldQueueIds);

		const removedIds = oldQueueIds.filter((id) => !newIdSet.has(id));
		const addedIds = queueIds.filter((id) => !oldIdSet.has(id));

		// Remove items from Muuri grid
		if (removedIds.length > 0) {
			const items = muuri.getItems();
			const removedItems = removedIds
				.map((id) => {
					const idx = oldQueueIds.indexOf(id);
					return idx >= 0 && idx < items.length ? items[idx] : null;
				})
				.filter((item): item is Muuri.Item => item !== null);

			// Unmount Svelte components before removing
			for (const item of removedItems) {
				unmount(item.getElement()!!);
			}

			muuri.remove(removedItems, { removeElements: true });
		}

		// Add new items to Muuri grid
		if (addedIds.length > 0) {
			const newElements = addedIds.map((id) => {
				const idx = queueIds.indexOf(id);
				const music = queue[idx];
				return createItem(music, id);
			});
			muuri.add(newElements);
		}

		// Update touch behavior for all queue items
		elementToggleDraggable();

		// Store current queue IDs for next comparison
		oldQueueIds = [...queueIds];
	}

	onMount(() => {
		initMuuri();
		$effect(refreshGrid);

		return () => {
			if (muuri) muuri.destroy(true);
		};
	});

	// Reactively update draggable state when dependencies change
	$effect(elementToggleDraggable);
</script>

<Sidebar type={SidebarType.Right} class="flex flex-col">
	<div class="grid grid-cols-[auto_max-content_max-content] items-center gap-3 p-3">
		<p class="text-[1.2rem] font-semibold md:text-[1.5rem]">Now Playing</p>
		<button class="w-7" onclick={cleanQueue}>
			<Icon type={IconType.CleanQueue} />
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
	<div class="scrollbar-hidden w-full flex-1 overflow-auto">
		<div class="relative w-full" bind:this={gridElement}></div>
	</div>
</Sidebar>
