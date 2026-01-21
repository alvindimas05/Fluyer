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

	// Drag and drop handlers
	function handleDragStart(e: DragEvent, index: number) {
		if (!draggingEnabled || index === musicStore.currentIndex) {
			e.preventDefault();
			return;
		}
		draggedIndex = index;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', index.toString());
		}
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'move';
		}
		dragOverIndex = index;
	}

	function handleDragLeave() {
		dragOverIndex = null;
	}

	async function handleDrop(e: DragEvent, toIndex: number) {
		e.preventDefault();
		dragOverIndex = null;

		if (draggedIndex === null || draggedIndex === toIndex) {
			draggedIndex = null;
			return;
		}

		const fromIndex = draggedIndex;
		draggedIndex = null;

		// Update queueIds to match new order
		let queueIds = [...musicStore.queueIds];
		const uuid = queueIds[fromIndex];
		queueIds.splice(fromIndex, 1);
		queueIds.splice(toIndex, 0, uuid);
		musicStore.queueIds = queueIds;

		await QueueService.moveTo(fromIndex, toIndex);
	}

	function handleDragEnd() {
		draggedIndex = null;
		dragOverIndex = null;
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
	<div class="scrollbar-hidden w-full flex-1 overflow-auto">
		<div class="relative w-full">
			{#each musicStore.queue as music, index (musicStore.queueIds[index])}
				{@const uuid = musicStore.queueIds[index]}
				{@const isPlaying = index === musicStore.currentIndex}
				{@const isDragOver = dragOverIndex === index}
				<div
					use:observeElement={uuid}
					role="listitem"
					class="w-full transition-transform duration-150"
					class:opacity-50={draggedIndex === index}
					class:border-t-2={isDragOver && draggedIndex !== null && draggedIndex < index}
					class:border-b-2={isDragOver && draggedIndex !== null && draggedIndex > index}
					class:border-blue-500={isDragOver}
					draggable={draggingEnabled && !isPlaying}
					ondragstart={(e) => handleDragStart(e, index)}
					ondragover={(e) => handleDragOver(e, index)}
					ondragleave={handleDragLeave}
					ondrop={(e) => handleDrop(e, index)}
					ondragend={handleDragEnd}
				>
					<MusicQueueItem {music} {uuid} visible={isSidebarVisible && visibleItems.has(uuid)} />
				</div>
			{/each}
		</div>
	</div>
</Sidebar>
