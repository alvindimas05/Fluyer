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
	class="pointer-events-none fixed left-0 top-0 z-50 grid h-12 w-full gap-y-2 px-3 sm:px-0 sm:pb-3
        {isMacos() ? 'sm:justify-end' : ''}
        {isMacos() ? 'right-0' : 'left-0'}
        animate__animated animate__slideInDown"
	style="margin-top: {isMobile() ? mobileStore.statusBarHeight : 8}px;
        grid-template-columns: {vm.state.gridSize};"
	bind:this={vm.element}
>
	<div class="pointer-events-auto h-full sm:mx-3">
		{#if playlistStore.isCreating}
			<div class="pointer-events-auto flex h-full gap-1">
				<Button
					class="grid h-full justify-center rounded p-[3.5px] sm:p-0"
					onclick={vm.confirmPlaylistCreation}
				>
					<div class="w-5">
						<Icon type={IconType.Check} />
					</div>
				</Button>
				<Button
					class="grid h-full justify-center rounded p-[3.5px] sm:p-0"
					onclick={vm.cancelPlaylistCreation}
				>
					<div class="w-5">
						<Icon type={IconType.Cancel} />
					</div>
				</Button>
			</div>
		{:else if musicStore.listType === 'playlist'}
			<Button
				class="pointer-events-auto grid h-full grid-cols-[auto_min-content] items-center gap-x-1 rounded px-2"
				onclick={vm.startPlaylistCreation}
			>
				<div>Create Playlist</div>
				<div class="w-5">
					<Icon type={IconType.PlaylistAdd} />
				</div>
			</Button>
		{/if}
	</div>
	<div
		class="pointer-events-none grid h-fit grid-cols-[min-content_1fr]
        gap-x-2
        sm:mx-3 sm:h-auto sm:gap-x-4
        {isMacos() ? 'justify-end' : 'justify-start'}"
	>
		<div>
			<Button
				class="pointer-events-auto grid aspect-square h-full justify-center rounded p-[3.5px] sm:p-0"
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
		</div>
		<Toggle
			class="pointer-events-auto h-full w-full"
			iconStyle="width: {vm.iconSize}px;"
			options={vm.musicListOptions}
			selected={musicStore.listType}
			onchange={vm.handleToggleChange}
		/>
	</div>
	<Input
		class="pointer-events-auto h-fit p-0 sm:mx-3 sm:h-full
            {isMacos() ? 'order-first sm:order-last' : 'order-first'} rounded"
		icon={IconType.Search}
		placeholder="Search..."
		bind:value={filterStore.search}
	/>
</div>
