<script lang="ts">
	import MusicItem from './MusicItem.svelte';
	import { useMusicList } from '../viewmodels/useMusicList.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';

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
			destroy() {
				observer?.unobserve(node);
			}
		};
	}

	function shouldHideItem(index: number): boolean {
		const indexInRow = index % vm.state.columnCount;
		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount / 2;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount / 2;
		}
		return false;
	}
</script>

<svelte:window onresize={vm.updateSize} />

<div class="relative w-full h-full px-3 overflow-y-auto scrollbar-hidden">
	{#if vm.data && vm.data.length > 0 && vm.state.columnCount}
		<div
			class="grid gap-x-6"
			style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
		>
			{#each vm.data as item, index}
				{@const itemKey = getItemKey(item)}
				<div
					use:observeElement={itemKey}
					class="animate__animated {shouldHideItem(index) ? 'animate__fadeOut' : 'animate__fadeIn'}"
					style="animation-duration: {isLinux() ? '350ms' : '500ms'}; {shouldHideItem(index)
						? 'pointer-events: none;'
						: ''}"
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
