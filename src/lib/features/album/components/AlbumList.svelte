<script lang="ts">
	import { VList } from 'virtua/svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import AlbumItem from '$lib/features/album/components/AlbumItem.svelte';
	import { useAlbumList } from '$lib/features/album/viewmodels/useAlbumList.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';

	const vm = useAlbumList();

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

	function shouldHideGridItem(indexInRow: number): boolean {
		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount;
		}
		return false;
	}
</script>

<svelte:window onresize={vm.updateItemWidth} />

<div
	class="w-full"
	style="height: {vm.isHorizontal
		? vm.itemHeight
		: window.innerHeight - filterStore.bar.height - playerBarStore.height}px;"
>
	{#key vm.isHorizontal}
		<VList
			onwheel={vm.isHorizontal ? vm.onMouseWheel : undefined}
			data={vm.data}
			class="scrollbar-hidden {vm.isHorizontal ? '' : 'overflow-y-clip'}"
			horizontal={vm.isHorizontal}
			style="padding-bottom: {vm.isHorizontal
				? 0
				: mobileStore.navigationBarHeight + mobileStore.statusBarHeight}px;"
			getKey={(_, i) => i}
			bind:this={vm.virtualizerHandle}
			onscroll={(offset: number) => vm.saveScrollOffset(offset)}
		>
			{#snippet children(dataList, index)}
				{#if vm.isHorizontal}
					<div
						class="animate__animated {shouldHideHorizontalItem(index)
							? 'animate__fadeOut'
							: 'animate__fadeIn'}"
						style="width: {vm.state.itemWidth}px; animation-duration: {isLinux()
							? '350ms'
							: '500ms'}; {shouldHideHorizontalItem(index) ? 'pointer-events: none;' : ''}"
					>
						<AlbumItem musicList={dataList} {index} />
					</div>
				{:else}
					<div class="flex">
						{#each dataList as musicList, dataIndex}
							<div
								class="animate__animated {shouldHideGridItem(dataIndex)
									? 'animate__fadeOut'
									: 'animate__fadeIn'}"
								style="width: {vm.state.itemWidth}px; animation-duration: {isLinux()
									? '350ms'
									: '500ms'}; {shouldHideGridItem(dataIndex) ? 'pointer-events: none;' : ''}"
							>
								<AlbumItem {musicList} index={index * vm.state.columnCount + dataIndex} />
							</div>
						{/each}
					</div>
				{/if}
			{/snippet}
		</VList>
	{/key}
</div>
