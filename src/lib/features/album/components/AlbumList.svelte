<script lang="ts">
	import mobileStore from '$lib/stores/mobile.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import AlbumItem from '$lib/features/album/components/AlbumItem.svelte';
	import { useAlbumList } from '$lib/features/album/viewmodels/useAlbumList.svelte';

	const vm = useAlbumList();

	let scrollContainer = $state<HTMLDivElement>();
</script>

<svelte:window onresize={vm.updateItemWidth} />

<div
	class="w-screen"
	style="height: {vm.filteredItemCount === 0
		? 0
		: vm.isHorizontal
			? vm.itemHeight
			: window.innerHeight -
				filterStore.bar.height -
				playerBarStore.height -
				mobileStore.navigationBarHeight -
				mobileStore.statusBarHeight}px;"
>
	{#if vm.isHorizontal}
		<!-- Horizontal layout -->
		<div
			bind:this={scrollContainer}
			use:vm.scrollable
			class="linux-hardware-accelerate scrollbar-hidden flex h-full overflow-x-auto"
			onscroll={vm.handleScroll}
			onwheel={(e) => vm.handleWheel(e, scrollContainer)}
			style="padding-bottom: 0;"
		>
			{#each vm.data as musicList, index}
				{@const hiddenBySidebar = vm.shouldHideHorizontalItem(index)}
				{@const visibleByFilter = vm.isVisibleByFilter(musicList)}
				{@const inViewport = vm.visibleItems.has(index)}
				{@const shouldRender = vm.shouldRenderHorizontalItem(index, musicList)}
				<div
					use:vm.observeElement={index}
					class="linux-prevent-flicker flex-shrink-0 {inViewport
						? hiddenBySidebar
							? 'animate__animated animate__fadeOut'
							: 'animate__animated animate__fadeIn'
						: ''}"
					style="width: {vm.state.itemWidth}px; animation-duration: 500ms; {hiddenBySidebar
						? 'pointer-events: none; opacity: 0;'
						: 'opacity: 1;'}"
					style:display={visibleByFilter ? undefined : 'none'}
					onanimationend={() => vm.handleAnimationEnd(index, hiddenBySidebar)}
				>
					{#if shouldRender}
						<AlbumItem {musicList} {index} visible={inViewport} />
					{/if}
				</div>
			{/each}
		</div>
	{:else}
		<!-- Grid layout -->
		<div
			bind:this={scrollContainer}
			use:vm.scrollable
			onscroll={vm.handleScroll}
			class="scrollbar-hidden h-full overflow-y-auto"
			style="padding-bottom: {mobileStore.navigationBarHeight + mobileStore.statusBarHeight}px;"
		>
			<div
				class="grid"
				style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
			>
				{#each vm.data as musicList, index}
					{@const hiddenBySidebar = vm.shouldHideGridItem(index)}
					{@const visibleByFilter = vm.isVisibleByFilter(musicList)}
					{@const inViewport = vm.visibleItems.has(index)}
					{@const shouldRender = vm.shouldRenderGridItem(index, musicList)}
					<div
						use:vm.observeElement={index}
						class="linux-prevent-flicker {inViewport
							? hiddenBySidebar
								? 'animate__animated animate__fadeOut'
								: 'animate__animated animate__fadeIn'
							: ''}"
						style="width: {vm.state.itemWidth}px; animation-duration: 500ms; {hiddenBySidebar
							? 'pointer-events: none; opacity: 0;'
							: 'opacity: 1;'}"
						style:display={visibleByFilter ? undefined : 'none'}
						onanimationend={() => vm.handleAnimationEnd(index, hiddenBySidebar)}
					>
						{#if shouldRender}
							<AlbumItem {musicList} {index} visible={inViewport} />
						{/if}
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
