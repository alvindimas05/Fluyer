<script lang="ts">
	import mobileStore from '$lib/stores/mobile.svelte';

	import filterStore from '$lib/stores/filter.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import AlbumItem from '$lib/features/album/components/AlbumItem.svelte';
	import { useAlbumList } from '$lib/features/album/viewmodels/useAlbumList.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';
	import ToastService from '$lib/services/ToastService.svelte';

	import { type MusicData } from '$lib/features/music/types';

	const vm = useAlbumList();

	// Track visibility of items using IntersectionObserver
	let visibleItems = $state<Set<number>>(new Set());
	let observer: IntersectionObserver | null = null;

	// Track items that are animating out (hidden by sidebar)
	let animatingOutItems = $state<Set<number>>(new Set());

	// Handle sidebar fadeout animation completion
	function handleAnimationEnd(index: number, isHiddenBySidebar: boolean) {
		if (isHiddenBySidebar) {
			animatingOutItems = new Set([...animatingOutItems, index]);
		}
	}

	// Check if item should render based on visibility conditions (horizontal)
	function shouldRenderHorizontalItem(index: number, musicList: MusicData[]): boolean {
		// If not visible by filter, don't render
		if (!isVisibleByFilter(musicList)) return false;

		// If not in visibleItems (outside viewport), don't render
		if (!visibleItems.has(index)) return false;

		// If hidden by sidebar and animation completed, don't render
		if (shouldHideHorizontalItem(index) && animatingOutItems.has(index)) return false;

		return true;
	}

	// Check if item should render based on visibility conditions (grid)
	function shouldRenderGridItem(index: number, musicList: MusicData[]): boolean {
		// If not visible by filter, don't render
		if (!isVisibleByFilter(musicList)) return false;

		// If not in visibleItems (outside viewport), don't render
		if (!visibleItems.has(index)) return false;

		// If hidden by sidebar and animation completed, don't render
		if (shouldHideGridItem(index) && animatingOutItems.has(index)) return false;

		return true;
	}

	function isVisibleByFilter(musicList: MusicData[]) {
		const search = filterStore.search.toLowerCase();
		const firstItem = musicList[0];

		if (!filterStore.search && !filterStore.album) return true;

		return (
			(filterStore.album && firstItem.album === filterStore.album.name) ||
			firstItem.album?.toLowerCase().includes(search) ||
			firstItem.albumArtist?.toLowerCase().includes(search)
		);
	}

	const visualIndices = $derived.by(() => {
		// Reactively depend on filter properties
		filterStore.search;
		filterStore.album;

		const map = new Map<number, number>();
		let count = 0;

		if (vm.data) {
			vm.data.forEach((item, index) => {
				if (isVisibleByFilter(item)) {
					map.set(index, count++);
				}
			});
		}
		return map;
	});

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
	let toastWidth = $derived(vm.state.itemWidth * 2); // User requested 2 items hidden

	const extraToleranceWidth = 10;
	function shouldHideHorizontalItem(index: number): boolean {
		if (!visualIndices.has(index)) return true;

		const visualIndex = visualIndices.get(index)!;

		// Calculate item's position relative to viewport using visual index
		const itemLeft = visualIndex * vm.state.itemWidth - vm.state.scrollLeft;
		const itemRight = itemLeft + vm.state.itemWidth;
		const viewportWidth = window.innerWidth;

		if (sidebarStore.showType === SidebarType.Left) {
			// Hide if item overlaps with left sidebar area
			if (itemLeft < sidebarWidth - extraToleranceWidth) return true;
		}

		if (sidebarStore.showType === SidebarType.Right) {
			// Hide if item overlaps with right sidebar area
			if (itemRight > viewportWidth - sidebarWidth + extraToleranceWidth) return true;
		}

		// Hide items for Toasts (Right side)
		if (ToastService.toasts.length > 0 && vm.state.columnCount > 2) {
			if (itemRight > viewportWidth - toastWidth + extraToleranceWidth) return true;
		}

		return false;
	}

	function shouldHideGridItem(index: number): boolean {
		if (!visualIndices.has(index)) return true;

		const visualIndex = visualIndices.get(index)!;
		const indexInRow = visualIndex % vm.state.columnCount;

		if (sidebarStore.showType === SidebarType.Left) {
			if (indexInRow < sidebarStore.hiddenColumnCount) return true;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			if (indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount) return true;
		}

		// Hide items for Toasts (Right side)
		if (ToastService.toasts.length > 0 && vm.state.columnCount > 2) {
			if (indexInRow >= vm.state.columnCount - 2) return true; // User requested 2 items
		}

		return false;
	}

	// Reset animating out state when item becomes visible by sidebar
	$effect(() => {
		if (vm.data) {
			vm.data.forEach((_, index) => {
				const isHidden = vm.isHorizontal
					? shouldHideHorizontalItem(index)
					: shouldHideGridItem(index);
				if (!isHidden && animatingOutItems.has(index)) {
					animatingOutItems = new Set([...animatingOutItems].filter((i) => i !== index));
				}
			});
		}
	});

	let scrollContainer = $state<HTMLDivElement>();

	function handleScroll(e: Event) {
		const target = e.target as HTMLDivElement;
		vm.state.scrollLeft = target.scrollLeft;
		vm.state.scrollTop = target.scrollTop;
	}

	$effect(() => {
		if (scrollContainer) {
			scrollContainer.scrollLeft = vm.state.scrollLeft;
			scrollContainer.scrollTop = vm.state.scrollTop;
		}
	});

	function handleWheel(e: WheelEvent) {
		if (vm.isHorizontal && e.deltaX === 0) {
			e.preventDefault();
			if (scrollContainer) scrollContainer.scrollLeft += e.deltaY;
		}
	}
</script>

<svelte:window onresize={vm.updateItemWidth} />

<div
	class="w-screen"
	style="height: {vm.isHorizontal
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
			class="linux-hardware-accelerate scrollbar-hidden flex h-full overflow-x-auto"
			onscroll={handleScroll}
			onwheel={handleWheel}
			style="padding-bottom: 0;"
		>
			{#each vm.data as musicList, index}
				{@const hiddenBySidebar = shouldHideHorizontalItem(index)}
				{@const visibleByFilter = isVisibleByFilter(musicList)}
				{@const inViewport = visibleItems.has(index)}
				{@const shouldRender = shouldRenderHorizontalItem(index, musicList)}
				<div
					use:observeElement={index}
					class="linux-prevent-flicker flex-shrink-0 {hiddenBySidebar &&
					inViewport &&
					visibleByFilter
						? 'animate__animated animate__fadeOut'
						: ''}"
					style="width: {vm.state.itemWidth}px; animation-duration: 500ms; {hiddenBySidebar
						? 'pointer-events: none; opacity: 0;'
						: 'opacity: 1;'}"
					style:display={visibleByFilter ? undefined : 'none'}
					onanimationend={() => handleAnimationEnd(index, hiddenBySidebar)}
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
			onscroll={handleScroll}
			class="scrollbar-hidden h-full overflow-y-auto"
			style="padding-bottom: {mobileStore.navigationBarHeight + mobileStore.statusBarHeight}px;"
		>
			<div
				class="grid"
				style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
			>
				{#each vm.data as musicList, index}
					{@const hiddenBySidebar = shouldHideGridItem(index)}
					{@const visibleByFilter = isVisibleByFilter(musicList)}
					{@const inViewport = visibleItems.has(index)}
					{@const shouldRender = shouldRenderGridItem(index, musicList)}
					<div
						use:observeElement={index}
						class={isLinux()
							? ''
							: hiddenBySidebar && inViewport && visibleByFilter
								? 'animate__animated animate__fadeOut'
								: ''}
						style="width: {vm.state.itemWidth}px; {isLinux()
							? ''
							: 'animation-duration: 500ms;'} {hiddenBySidebar
							? 'pointer-events: none; opacity: 0;'
							: 'opacity: 1;'}"
						style:display={visibleByFilter ? undefined : 'none'}
						onanimationend={() => handleAnimationEnd(index, hiddenBySidebar)}
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
