<script lang="ts">
	import mobileStore from '$lib/stores/mobile.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import AlbumItem from '$lib/features/album/components/AlbumItem.svelte';
	import { useAlbumList } from '$lib/features/album/viewmodels/useAlbumList.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';

	const vm = useAlbumList();

	// Track visibility of items using IntersectionObserver
	let visibleItems = $state<Set<number>>(new Set());
	let observer: IntersectionObserver | null = null;

	function observeElement(node: HTMLElement, index: number) {
		if (!observer) {
			observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						const itemIndex = entry.target.getAttribute('data-item-index');
						if (itemIndex !== null) {
							if (entry.isIntersecting) {
								visibleItems = new Set([...visibleItems, parseInt(itemIndex)]);
							}
						}
					});
				},
				{ threshold: 0 }
			);
		}

		node.setAttribute('data-item-index', index.toString());
		observer.observe(node);

		return {
			destroy() {
				observer?.unobserve(node);
			}
		};
	}

	// Calculate sidebar width (2 columns)
	let sidebarWidth = $derived(vm.state.itemWidth * sidebarStore.hiddenColumnCount);

	const extraToleranceWidth = 10;
	function shouldHideHorizontalItem(index: number): boolean {
		if (!sidebarStore.showType) return false;

		// Calculate item's position relative to viewport
		const itemLeft = index * vm.state.itemWidth - vm.state.scrollLeft;
		const itemRight = itemLeft + vm.state.itemWidth;
		const viewportWidth = window.innerWidth;

		if (sidebarStore.showType === SidebarType.Left) {
			// Hide if item overlaps with left sidebar area
			return itemLeft < sidebarWidth - extraToleranceWidth;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			// Hide if item overlaps with right sidebar area
			return itemRight > viewportWidth - sidebarWidth + extraToleranceWidth;
		}
		return false;
	}

	function shouldHideGridItem(index: number): boolean {
		const indexInRow = index % vm.state.columnCount;
		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount;
		}
		return false;
	}

	let scrollContainer: HTMLDivElement;

	function handleScroll(e: Event) {
		const target = e.target as HTMLDivElement;
		vm.state.scrollLeft = target.scrollLeft;
	}

	function handleWheel(e: WheelEvent) {
		if (vm.isHorizontal && e.deltaX === 0) {
			e.preventDefault();
			scrollContainer.scrollLeft += e.deltaY;
		}
	}
</script>

<svelte:window onresize={vm.updateItemWidth} />

<div
	class="w-screen"
	style="height: {vm.isHorizontal
		? vm.itemHeight
		: window.innerHeight - filterStore.bar.height - playerBarStore.height}px;"
>
	{#if vm.isHorizontal}
		<!-- Horizontal layout -->
		<div
			bind:this={scrollContainer}
			class="flex h-full overflow-x-auto scrollbar-hidden"
			onscroll={handleScroll}
			onwheel={handleWheel}
			style="padding-bottom: 0;"
		>
			{#each vm.data as musicList, index}
				<div
					use:observeElement={index}
					class="flex-shrink-0 animate__animated {shouldHideHorizontalItem(index)
						? 'animate__fadeOut'
						: 'animate__fadeIn'}"
					style="width: {vm.state.itemWidth}px; animation-duration: {isLinux()
						? '350ms'
						: '500ms'}; {shouldHideHorizontalItem(index) ? 'pointer-events: none;' : ''}"
				>
					<AlbumItem {musicList} {index} visible={visibleItems.has(index)} />
				</div>
			{/each}
		</div>
	{:else}
		<!-- Grid layout -->
		<div
			class="h-full overflow-y-auto scrollbar-hidden"
			style="padding-bottom: {mobileStore.navigationBarHeight + mobileStore.statusBarHeight}px;"
		>
			<div
				class="grid"
				style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
			>
				{#each vm.data as musicList, index}
					<div
						use:observeElement={index}
						class="animate__animated {shouldHideGridItem(index)
							? 'animate__fadeOut'
							: 'animate__fadeIn'}"
						style="width: {vm.state.itemWidth}px; animation-duration: {isLinux()
							? '350ms'
							: '500ms'}; {shouldHideGridItem(index) ? 'pointer-events: none;' : ''}"
					>
						<AlbumItem {musicList} {index} visible={visibleItems.has(index)} />
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
