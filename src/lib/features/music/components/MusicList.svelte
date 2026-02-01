<script lang="ts">
	import MusicItem from './MusicItem.svelte';
	import { useMusicList } from '../viewmodels/useMusicList.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';

	import filterStore from '$lib/stores/filter.svelte';

	const vm = useMusicList();

	// Track visibility of items using IntersectionObserver
	let visibleItems = $state<Set<string>>(new Set());
	let observer: IntersectionObserver | null = null;

	function getItemKey(item: any): string {
		if ('duration' in item) {
			return `music-${item.path}`;
		}
		return `folder-${item.path}`;
	}

	function isVisible(item: any): boolean {
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

	function observeElement(node: HTMLElement, key: string) {
		if (!observer) {
			observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						const itemKey = entry.target.getAttribute('data-item-key');
						if (itemKey) {
							if (entry.isIntersecting) {
								visibleItems = new Set([...visibleItems, itemKey]);
							}
						}
					});
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

	function shouldHideItem(index: number): boolean {
		if (!visualIndices.has(index)) return true; // Should be hidden anyway if not in visual map

		const visualIndex = visualIndices.get(index)!;
		const indexInRow = visualIndex % vm.state.columnCount;

		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount / 2;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount / 2;
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
	class="scrollbar-hidden relative h-full w-full overflow-y-auto px-3"
>
	{#if vm.data && vm.data.length > 0 && vm.state.columnCount}
		<div
			class="grid gap-x-6"
			style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
		>
			{#each vm.data as item, index}
				{@const itemKey = getItemKey(item)}
				<!-- FIXME: Animation is stuttering because of reactive and re-rendering on Sidebar change -->
				<div
					use:observeElement={itemKey}
					class="{isLinux() ? '' : 'animate__animated'}
					{!isLinux() && shouldHideItem(index) ? 'animate__fadeOut' : 'animate__fadeIn'}
					{isLinux() && shouldHideItem(index) ? 'invisible' : ''}"
					style="animation-duration: 500ms; {shouldHideItem(index) ? 'pointer-events: none;' : ''}"
					style:display={isVisible(item) ? undefined : 'none'}
				>
					{#if 'duration' in item}
						<MusicItem music={item} visible={visibleItems.has(itemKey)} />
					{:else}
						<MusicItem folder={item} visible={visibleItems.has(itemKey)} />
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>
