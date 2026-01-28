<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import { isMobile } from '$lib/platform';
	import Sidebar from '$lib/features/sidebar/components/Sidebar.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import MusicQueueItem from '$lib/features/music_queue/components/MusicQueueItem.svelte';
	import { useMusicQueueList } from '$lib/features/music_queue/viewmodels/useMusicQueueList.svelte';

	const vm = useMusicQueueList();
</script>

<Sidebar type={SidebarType.Right} class="flex flex-col">
	<div class="grid grid-cols-[auto_max-content_max-content] items-center gap-3 p-3">
		<p class="text-[1.2rem] font-semibold md:text-[1.5rem]">Now Playing</p>
		<button class="w-7" onclick={vm.cleanQueue}>
			<Icon type={IconType.CleanQueue} />
		</button>
		{#if isMobile()}
			<button class="w-7" onclick={vm.dragToggle}>
				{#if vm.draggingEnabled}
					<Icon type={IconType.DragOff} />
				{:else}
					<Icon type={IconType.DragOn} />
				{/if}
			</button>
		{/if}
	</div>
	<div bind:this={vm.scrollContainer} class="scrollbar-hidden w-full flex-1 overflow-auto">
		<div class="relative w-full">
			{#each musicStore.queue as music, index (musicStore.queueIds[index])}
				{@const uuid = musicStore.queueIds[index]}
				{@const isPlaying = index === musicStore.currentIndex}
				{@const isDragging = vm.draggedIndex === index}
				{@const showDropIndicator =
					vm.dragOverIndex === index && vm.draggedIndex !== null && vm.draggedIndex !== index}
				<div
					use:vm.observeElement={uuid}
					use:vm.registerItem={index}
					role="listitem"
					class="w-full select-none"
					class:z-50={isDragging}
					class:shadow-lg={isDragging}
					class:cursor-grabbing={isDragging}
					class:cursor-grab={vm.draggingEnabled && !isPlaying && !isDragging}
					style:transform={vm.getDragTransform(index)}
					style:transition={isDragging ? 'none' : 'transform 150ms ease'}
					style:touch-action={vm.draggingEnabled && !isPlaying ? 'none' : 'auto'}
					onpointerdown={(e) => vm.handlePointerDown(e, index)}
					onpointermove={vm.handlePointerMove}
					onpointerup={vm.handlePointerUp}
					onpointercancel={vm.handlePointerCancel}
				>
					{#if showDropIndicator && vm.draggedIndex !== null && vm.draggedIndex > index}
						<div class="h-1 w-full bg-white bg-opacity-20"></div>
					{/if}
					<MusicQueueItem
						{music}
						{uuid}
						visible={vm.isSidebarVisible && vm.visibleItems.has(uuid)}
					/>
					{#if showDropIndicator && vm.draggedIndex !== null && vm.draggedIndex < index}
						<div class="h-1 w-full bg-white bg-opacity-20"></div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</Sidebar>
