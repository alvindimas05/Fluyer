<script lang="ts">
	import mobileStore from '$lib/stores/mobile.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import AlbumItem from '$lib/features/album/components/AlbumItem.svelte';
	import { useAlbumList } from '$lib/features/album/viewmodels/useAlbumList.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';

	import { type MusicData } from '$lib/features/music/types';

	const vm = useAlbumList();

	// Track visibility of items using IntersectionObserver
	let visibleItems = $state<Set<number>>(new Set());
	let observer: IntersectionObserver | null = null;

	function isVisible(musicList: MusicData[]) {
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
		const map = new Map<number, number>();
		let count = 0;
		// Reactively depend on filter properties
		const search = filterStore.search;
		const album = filterStore.album;

		if (vm.data) {
			vm.data.forEach((item, index) => {
				if (isVisible(item)) {
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

	const extraToleranceWidth = 10;
	function shouldHideHorizontalItem(index: number): boolean {
		if (!sidebarStore.showType) return false;
		if (!visualIndices.has(index)) return true;

		const visualIndex = visualIndices.get(index)!;

		// Calculate item's position relative to viewport using visual index
		const itemLeft = visualIndex * vm.state.itemWidth - vm.state.scrollLeft;
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
		if (!visualIndices.has(index)) return true;

		const visualIndex = visualIndices.get(index)!;
		const indexInRow = visualIndex % vm.state.columnCount;

		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount;
		}
		return false;
	}

	let scrollContainer = $state<HTMLDivElement>();

	function handleScroll(e: Event) {
		const target = e.target as HTMLDivElement;
		vm.state.scrollLeft = target.scrollLeft;
	}

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
		: window.innerHeight - filterStore.bar.height - playerBarStore.height}px;"
>
	{#if vm.isHorizontal}
		<!-- Horizontal layout -->
		<div
			bind:this={scrollContainer}
			class="scrollbar-hidden flex h-full overflow-x-auto"
			onscroll={handleScroll}
			onwheel={handleWheel}
			style="padding-bottom: 0;"
		>
			{#each vm.data as musicList, index}
				<div
					use:observeElement={index}
					class="animate__animated flex-shrink-0 {shouldHideHorizontalItem(index)
						? 'animate__fadeOut'
						: 'animate__fadeIn'}"
					style="width: {vm.state.itemWidth}px; animation-duration: {isLinux()
						? '350ms'
						: '500ms'}; {shouldHideHorizontalItem(index) ? 'pointer-events: none;' : ''}"
					style:display={isVisible(musicList) ? undefined : 'none'}
				>
					<AlbumItem {musicList} {index} visible={visibleItems.has(index)} />
				</div>
			{/each}
		</div>
	{:else}
		<!-- Grid layout -->
		<div
			class="scrollbar-hidden h-full overflow-y-auto"
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
						style:display={isVisible(musicList) ? undefined : 'none'}
					>
						<AlbumItem {musicList} {index} visible={visibleItems.has(index)} />
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
