<script lang="ts">
	import { VList } from 'virtua/svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import AlbumItem from '$lib/features/album/components/AlbumItem.svelte';
	import { useAlbumList } from '$lib/features/album/viewmodels/useAlbumList.svelte';

	const vm = useAlbumList();
</script>

<svelte:window onresize={vm.updateItemWidth} />

<div class="w-full" style="height: {vm.isHorizontal ? vm.itemHeight : window.innerHeight}px;">
	{#key vm.isHorizontal}
		<VList
			onwheel={vm.isHorizontal ? vm.onMouseWheel : undefined}
			data={vm.data}
			class="scrollbar-hidden {vm.isHorizontal ? '' : 'overflow-y-clip'}"
			horizontal={vm.isHorizontal}
			style="padding-bottom: {vm.isHorizontal
				? 0
				: playerBarStore.height +
					filterStore.bar.height +
					mobileStore.navigationBarHeight +
					mobileStore.statusBarHeight}px;"
			getKey={(_, i) => i}
			bind:this={vm.virtualizerHandle}
		>
			{#snippet children(dataList, index)}
				{#if vm.isHorizontal}
					<div style="width: {vm.state.itemWidth}px;">
						<AlbumItem musicList={dataList} {index} />
					</div>
				{:else}
					<div class="flex">
						{#each dataList as musicList, dataIndex}
							<div style="width: {vm.state.itemWidth}px;">
								<AlbumItem {musicList} index={index * vm.state.columnCount + dataIndex} />
							</div>
						{/each}
					</div>
				{/if}
			{/snippet}
		</VList>
	{/key}
</div>
