<script lang="ts">
	import MusicItem from './MusicItem.svelte';
	import { VList } from 'virtua/svelte';
	import { useMusicList } from '../viewmodels/useMusicList.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { isLinux } from '$lib/platform';

	const vm = useMusicList();

	function shouldHideItem(indexInRow: number): boolean {
		if (sidebarStore.showType === SidebarType.Left) {
			return indexInRow < sidebarStore.hiddenColumnCount / 2;
		}
		if (sidebarStore.showType === SidebarType.Right) {
			return indexInRow >= vm.state.columnCount - sidebarStore.hiddenColumnCount / 2;
		}
		return false;
	}

	function chunkData(data: any[], columnCount: number) {
		const rows = [];
		for (let i = 0; i < data.length; i += columnCount) {
			rows.push(data.slice(i, i + columnCount));
		}
		return rows;
	}

	let chunkedData = $derived(vm.data ? chunkData(vm.data, vm.state.columnCount) : []);
</script>

<svelte:window onresize={vm.updateSize} />

<div class="h-full px-3">
	{#if chunkedData.length > 0 && vm.state.columnCount}
		{#key chunkedData}
			<VList
				class="scrollbar-hidden"
				data={chunkedData}
				getKey={(_, i) => i}
				style="padding-bottom: {playerBarStore.height}px;"
			>
				{#snippet children(list)}
					<div
						class="grid gap-x-6"
						style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
					>
						{#each list as item, indexInRow}
							{#if !shouldHideItem(indexInRow)}
								<div
									class="animate__animated animate__fadeIn"
									style="animation-duration: {isLinux() ? '350ms' : '500ms'};"
								>
									{#if 'duration' in item}
										<MusicItem music={item} />
									{:else}
										<MusicItem folder={item} />
									{/if}
								</div>
							{:else}
								<div
									class="animate__animated animate__fadeOut"
									style="animation-duration: 300ms;"
								></div>
							{/if}
						{/each}
					</div>
				{/snippet}
			</VList>
		{/key}
	{/if}
</div>
