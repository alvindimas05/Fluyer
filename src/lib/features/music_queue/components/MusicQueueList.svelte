<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import { isMobile } from '$lib/platform';
	import Sidebar from '$lib/features/sidebar/components/Sidebar.svelte';
	import ProgressService from '$lib/services/ProgressService.svelte';
	import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import QueueService from '$lib/services/QueueService.svelte';
	import MusicQueueItem from '$lib/features/music_queue/components/MusicQueueItem.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';

	let draggingEnabled = $state(!isMobile());
	let draggedIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);
	let dragOffsetY = $state(0);
	let currentY = $state(0);
	let itemRefs: HTMLElement[] = [];
	let scrollContainer: HTMLDivElement;

	// Track visibility of items using IntersectionObserver
	let visibleItems = $state<Set<string>>(new Set());
	let observer: IntersectionObserver | null = null;

	// Track if sidebar is visible
	let isSidebarVisible = $derived(sidebarStore.showType === SidebarType.Right);

	function cleanQueue() {
		MusicPlayerService.pause();
		ProgressService.reset();
	}

	function dragToggle() {
		draggingEnabled = !draggingEnabled;
	}

	function observeElement(node: HTMLElement, uuid: string) {
		if (!observer) {
			observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						const itemUuid = entry.target.getAttribute('data-uuid');
						if (itemUuid) {
							if (entry.isIntersecting) {
								visibleItems = new Set([...visibleItems, itemUuid]);
							} else {
								const newSet = new Set(visibleItems);
								newSet.delete(itemUuid);
								visibleItems = newSet;
							}
						}
					});
				},
				{ threshold: 0 }
			);
		}

		node.setAttribute('data-uuid', uuid);
		observer.observe(node);

		return {
			update(newUuid: string) {
				node.setAttribute('data-uuid', newUuid);
			},
			destroy() {
				observer?.unobserve(node);
			}
		};
	}

	// Register item ref
	function registerItem(node: HTMLElement, index: number) {
		itemRefs[index] = node;
		return {
			update(newIndex: number) {
				itemRefs[newIndex] = node;
			},
			destroy() {
				// Clean up reference
			}
		};
	}

	// Pointer-based drag handlers
	function handlePointerDown(e: PointerEvent, index: number) {
		if (!draggingEnabled || index === musicStore.currentIndex) return;

		const target = e.currentTarget as HTMLElement;
		target.setPointerCapture(e.pointerId);

		draggedIndex = index;
		const rect = target.getBoundingClientRect();
		dragOffsetY = e.clientY - rect.top;
		currentY = e.clientY;
	}

	function handlePointerMove(e: PointerEvent) {
		if (draggedIndex === null) return;

		currentY = e.clientY;

		// Calculate which item we're over
		const containerRect = scrollContainer.getBoundingClientRect();
		const relativeY = e.clientY - containerRect.top + scrollContainer.scrollTop;

		let newOverIndex = null;
		for (let i = 0; i < itemRefs.length; i++) {
			const item = itemRefs[i];
			if (!item) continue;
			const rect = item.getBoundingClientRect();
			const itemTop = rect.top - containerRect.top + scrollContainer.scrollTop;
			const itemMiddle = itemTop + rect.height / 2;

			if (relativeY < itemMiddle) {
				newOverIndex = i;
				break;
			}
			newOverIndex = i + 1;
		}

		if (newOverIndex !== null && newOverIndex !== draggedIndex) {
			dragOverIndex = Math.min(newOverIndex, musicStore.queue.length - 1);
		}
	}

	async function handlePointerUp(e: PointerEvent) {
		if (draggedIndex === null) return;

		const target = e.currentTarget as HTMLElement;
		target.releasePointerCapture(e.pointerId);

		const fromIndex = draggedIndex;
		const toIndex = dragOverIndex ?? draggedIndex;

		draggedIndex = null;
		dragOverIndex = null;

		if (fromIndex !== toIndex) {
			// Update queueIds to match new order
			let queueIds = [...musicStore.queueIds];
			const uuid = queueIds[fromIndex];
			queueIds.splice(fromIndex, 1);
			queueIds.splice(toIndex, 0, uuid);
			musicStore.queueIds = queueIds;

			await QueueService.moveTo(fromIndex, toIndex);
		}
	}

	function handlePointerCancel() {
		draggedIndex = null;
		dragOverIndex = null;
	}

	// Calculate transform for dragged item
	function getDragTransform(index: number): string {
		if (draggedIndex !== index) return '';

		const item = itemRefs[index];
		if (!item) return '';

		// Use offsetParent (the relative container) to calculate stable layout position
		// ignoring the current transform
		const parent = item.offsetParent as HTMLElement;
		if (!parent) return '';

		const parentRect = parent.getBoundingClientRect();
		// Layout top relative to viewport = parent viewport top + item offset top
		const currentLayoutTop = parentRect.top + item.offsetTop;

		const targetY = currentY - dragOffsetY;

		return `translateY(${targetY - currentLayoutTop}px)`;
	}
</script>

<Sidebar type={SidebarType.Right} class="flex flex-col">
	<div class="grid grid-cols-[auto_max-content_max-content] items-center gap-3 p-3">
		<p class="text-[1.2rem] font-semibold md:text-[1.5rem]">Now Playing</p>
		<button class="w-7" onclick={cleanQueue}>
			<Icon type={IconType.CleanQueue} />
		</button>
		{#if isMobile()}
			<button class="w-7" onclick={dragToggle}>
				{#if draggingEnabled}
					<Icon type={IconType.DragOff} />
				{:else}
					<Icon type={IconType.DragOn} />
				{/if}
			</button>
		{/if}
	</div>
	<div bind:this={scrollContainer} class="scrollbar-hidden w-full flex-1 overflow-auto">
		<div class="relative w-full">
			{#each musicStore.queue as music, index (musicStore.queueIds[index])}
				{@const uuid = musicStore.queueIds[index]}
				{@const isPlaying = index === musicStore.currentIndex}
				{@const isDragging = draggedIndex === index}
				{@const showDropIndicator =
					dragOverIndex === index && draggedIndex !== null && draggedIndex !== index}
				<div
					use:observeElement={uuid}
					use:registerItem={index}
					role="listitem"
					class="w-full select-none"
					class:z-50={isDragging}
					class:shadow-lg={isDragging}
					class:cursor-grabbing={isDragging}
					class:cursor-grab={draggingEnabled && !isPlaying && !isDragging}
					style:transform={getDragTransform(index)}
					style:transition={isDragging ? 'none' : 'transform 150ms ease'}
					onpointerdown={(e) => handlePointerDown(e, index)}
					onpointermove={handlePointerMove}
					onpointerup={handlePointerUp}
					onpointercancel={handlePointerCancel}
				>
					{#if showDropIndicator && draggedIndex !== null && draggedIndex > index}
						<div class="h-1 w-full bg-white bg-opacity-20"></div>
					{/if}
					<MusicQueueItem {music} {uuid} visible={isSidebarVisible && visibleItems.has(uuid)} />
					{#if showDropIndicator && draggedIndex !== null && draggedIndex < index}
						<div class="h-1 w-full bg-white bg-opacity-20"></div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</Sidebar>
