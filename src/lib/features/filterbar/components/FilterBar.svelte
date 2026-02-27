<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import { isMacos, isMobile } from '$lib/platform';
	import Toggle from '$lib/ui/components/Toggle.svelte';
	import Button from '$lib/ui/components/Button.svelte';
	import Input from '$lib/ui/components/Input.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import { filterBarStore } from '$lib/stores/filter.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import playlistStore from '$lib/stores/playlist.svelte';
	import { useFilterBar } from '../viewmodels/useFilterBar.svelte';

	const vm = useFilterBar();
</script>

<svelte:window onresize={vm.updateSize} />
<div
	class="pointer-events-none fixed left-0 top-0 z-50 grid w-full gap-y-2 px-3 sm:px-0 sm:pb-3
        {isMacos() ? 'sm:justify-end' : ''}
        {isMacos() ? 'right-0' : 'left-0'}
        animate__animated animate__slideInDown"
	style="margin-top: {isMobile() ? mobileStore.statusBarHeight : 8}px;
        grid-template-columns: {vm.state.gridSize};"
	bind:this={vm.element}
>
	<div
		class="grid h-9 gap-x-2 sm:contents
		{musicStore.listType === 'playlist' ? 'grid-cols-1' : 'grid-cols-[4fr_6fr]'}"
	>
		<div
			class="pointer-events-auto h-full sm:mx-3 sm:h-9
			{musicStore.listType === 'playlist' ? 'hidden sm:block' : ''}"
		></div>
		<div
			class="pointer-events-none grid h-full grid-cols-[min-content_minmax(0,1fr)] gap-x-2
	        sm:mx-3 sm:h-9"
		>
			<Button
				class="pointer-events-auto grid aspect-square h-full justify-center rounded sm:p-0"
				onclick={vm.toggleSort}
			>
				<div class="w-5">
					{#if filterBarStore.sortAsc}
						<Icon type={IconType.SortAsc} />
					{:else}
						<Icon type={IconType.SortDesc} />
					{/if}
				</div>
			</Button>

			{#if musicStore.listType === 'playlist'}
				<div class="pointer-events-auto flex h-full w-full min-w-0 justify-end sm:justify-start">
					{#if playlistStore.isCreating}
						<div class="grid h-full w-full grid-cols-[auto_auto] gap-x-2">
							<Button
								class="grid h-full w-full items-center justify-center gap-x-2 rounded p-[3.5px] sm:p-0 sm:px-2"
								onclick={vm.confirmPlaylistCreation}
							>
								<div class="w-5">
									<Icon type={IconType.Check} />
								</div>
							</Button>
							<Button
								class="grid h-full w-full items-center justify-center gap-x-2 rounded p-[3.5px] sm:p-0 sm:px-2"
								onclick={vm.cancelPlaylistCreation}
							>
								<div class="w-5">
									<Icon type={IconType.Cancel} />
								</div>
							</Button>
						</div>
					{:else}
						<Button
							class="flex h-full w-full items-center justify-between gap-x-1 overflow-hidden rounded px-2"
							onclick={vm.startPlaylistCreation}
						>
							<div class="truncate">Create Playlist</div>
							<div class="w-5 shrink-0">
								<Icon type={IconType.PlaylistAdd} />
							</div>
						</Button>
					{/if}
				</div>
			{:else}
				<Toggle
					class="pointer-events-auto h-full w-full"
					iconStyle="width: {vm.iconSize}px;"
					options={vm.musicListOptions}
					selected={musicStore.listType}
					onchange={vm.handleToggleChange}
				/>
			{/if}
		</div>
	</div>
	<Input
		class="pointer-events-auto h-fit p-0 sm:mx-3 sm:h-full
            {isMacos() ? 'order-first sm:order-last' : 'order-first'} rounded"
		icon={IconType.Search}
		placeholder="Search..."
		bind:value={filterStore.search}
	/>
</div>
