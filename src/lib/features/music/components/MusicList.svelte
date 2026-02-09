<script lang="ts">
	import MusicItem from './MusicItem.svelte';
	import { useMusicList } from '../viewmodels/useMusicList.svelte';

	const vm = useMusicList();
</script>

<svelte:window onresize={vm.updateSize} />

<div
	use:vm.scrollable
	onscroll={vm.handleScroll}
	class="linux-hardware-accelerate scrollbar-hidden relative h-full w-full overflow-y-auto px-3"
>
	{#if vm.data && vm.data.length > 0 && vm.state.columnCount}
		<div
			class="grid gap-x-6"
			style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
		>
			{#each vm.data as item, index}
				{@const itemKey = vm.getItemKey(item)}
				{@const hiddenBySidebar = vm.isHiddenBySidebar(index)}
				{@const visibleByFilter = vm.isVisibleByFilter(item)}
				{@const inViewport = vm.visibleItems.has(itemKey)}
				{@const shouldRender = vm.shouldRenderItem(itemKey, index, item)}
				<div
					use:vm.observeElement={itemKey}
					class="linux-prevent-flicker min-h-[64px] md:min-h-[72px] {inViewport
						? hiddenBySidebar
							? 'animate__animated animate__fadeOut'
							: 'animate__animated animate__fadeIn'
						: ''}"
					style="animation-duration: 500ms; {hiddenBySidebar
						? 'pointer-events: none; opacity: 0;'
						: 'opacity: 1;'}"
					style:display={visibleByFilter ? undefined : 'none'}
					onanimationend={() => vm.handleAnimationEnd(itemKey, hiddenBySidebar)}
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
