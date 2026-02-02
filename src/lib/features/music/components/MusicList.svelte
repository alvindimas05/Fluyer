<script lang="ts">
	import MusicItem from './MusicItem.svelte';
	import { useMusicList } from '../viewmodels/useMusicList.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import filterStore from '$lib/stores/filter.svelte';

	const vm = useMusicList();

	// Track visibility of items using IntersectionObserver
	let visibleItems = $state<Set<string>>(new Set());
	let observer: IntersectionObserver | null = null;

	// Track items that are animating out (hidden by sidebar)
	let animatingOutItems = $state<Set<string>>(new Set());

	function getItemKey(item: any): string {
		if ('duration' in item) {
			return `music-${item.path}`;
		}
		return `folder-${item.path}`;
	}

	// Handle sidebar fadeout animation completion
	function handleAnimationEnd(itemKey: string, isHiddenBySidebar: boolean) {
		if (isHiddenBySidebar) {
			animatingOutItems = new Set([...animatingOutItems, itemKey]);
		}
	}

	// Check if item should render based on all visibility conditions
	function shouldRenderItem(itemKey: string, index: number, item: any): boolean {
		// If not visible by filter, don't render
		if (!isVisibleByFilter(item)) return false;

		// If not in visibleItems (outside viewport), don't render
		if (!visibleItems.has(itemKey)) return false;

		// If hidden by sidebar and animation completed, don't render
		if (isHiddenBySidebar(index) && animatingOutItems.has(itemKey)) return false;

		return true;
	}

	// Reset animating out state when item becomes visible by sidebar
	$effect(() => {
		if (vm.data) {
			vm.data.forEach((item, index) => {
				const itemKey = getItemKey(item);
				if (!isHiddenBySidebar(index) && animatingOutItems.has(itemKey)) {
					animatingOutItems = new Set([...animatingOutItems].filter((k) => k !== itemKey));
				}
			});
		}
	});

	function isVisibleByFilter(item: any): boolean {
		const search = filterStore.search.toLowerCase();

		// Folder check
		if (!('duration' in item)) {
			// Folder
			return item.path.toLowerCase().includes(search);
		}

		// Music check
		const music = item;
		const album = filterStore.album;
		const hasSearch = search.length > 0;
		const matchesSearch =
			!hasSearch ||
			[music.album, music.title, music.artist, music.albumArtist].some((v) =>
				v?.toLowerCase().includes(search)
			);

		const hasAlbum = !!album;
		const matchesAlbum = !hasAlbum || album.name === music.album;

		return matchesSearch && matchesAlbum;
	}

	// Calculate visual indices for items that are visible references
	const visualIndices = $derived.by(() => {
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

	function observeElement(node: HTMLElement, key: string) {
		if (!observer) {
			observer = new IntersectionObserver(
				(entries) => {
					let newVisible = new Set(visibleItems);
					let changed = false;

					entries.forEach((entry) => {
						const itemKey = entry.target.getAttribute('data-item-key');
						if (itemKey) {
							if (entry.isIntersecting) {
								if (!newVisible.has(itemKey)) {
									newVisible.add(itemKey);
									changed = true;
								}
							} else {
								if (newVisible.has(itemKey)) {
									newVisible.delete(itemKey);
									changed = true;
								}
							}
						}
					});

					if (changed) {
						visibleItems = newVisible;
					}
				},
				{ threshold: 0 }
			);
		}

		node.setAttribute('data-item-key', key);
		observer.observe(node);

		return {
			update(newKey: string) {
				node.setAttribute('data-item-key', newKey);
				observer?.unobserve(node);
				observer?.observe(node);
			},
			destroy() {
				observer?.unobserve(node);
			}
		};
	}

	function isHiddenBySidebar(index: number): boolean {
		if (!visualIndices.has(index)) return true; // Should be hidden anyway if not in visual map

		const visualIndex = visualIndices.get(index)!;
		const indexInRow = visualIndex % vm.state.columnCount;

		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount / 2;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - Math.ceil(sidebarStore.hiddenColumnCount / 2);
		}
		return false;
	}

	let scrollContainer = $state<HTMLDivElement>();

	function handleScroll(e: Event) {
		const target = e.target as HTMLDivElement;
		vm.state.scrollTop = target.scrollTop;
	}

	$effect(() => {
		if (scrollContainer) {
			scrollContainer.scrollTop = vm.state.scrollTop;
		}
	});
</script>

<svelte:window onresize={vm.updateSize} />

<div
	bind:this={scrollContainer}
	onscroll={handleScroll}
	class="linux-hardware-accelerate scrollbar-hidden relative h-full w-full overflow-y-auto px-3"
>
	{#if vm.data && vm.data.length > 0 && vm.state.columnCount}
		<div
			class="grid gap-x-6"
			style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
		>
			{#each vm.data as item, index}
				{@const itemKey = getItemKey(item)}
				{@const hiddenBySidebar = isHiddenBySidebar(index)}
				{@const visibleByFilter = isVisibleByFilter(item)}
				{@const inViewport = visibleItems.has(itemKey)}
				{@const shouldRender = shouldRenderItem(itemKey, index, item)}
				<div
					use:observeElement={itemKey}
					class="linux-prevent-flicker min-h-[64px] md:min-h-[72px] {inViewport
						? hiddenBySidebar
							? 'animate__animated animate__fadeOut'
							: 'animate__animated animate__fadeIn'
						: ''}"
					style="animation-duration: 500ms; {hiddenBySidebar ? 'pointer-events: none;' : ''}"
					style:display={visibleByFilter ? undefined : 'none'}
					onanimationend={() => handleAnimationEnd(itemKey, hiddenBySidebar)}
				>
					{#if shouldRender}
						{#if 'duration' in item}
							<MusicItem music={item} visible={inViewport} />
						{:else}
							<MusicItem folder={item} visible={inViewport} />
						{/if}
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>
