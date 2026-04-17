<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import { isLinux, isMacos, isMobile, isWindows } from '$lib/platform';
	import Toggle from '$lib/ui/components/Toggle.svelte';
	import Button from '$lib/ui/components/Button.svelte';
	import Input from '$lib/ui/components/Input.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import { filterBarStore } from '$lib/stores/filter.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import playlistStore from '$lib/stores/playlist.svelte';
	import { useFilterBar } from '../viewmodels/useFilterBar.svelte';
	import modalStore from '$lib/stores/modal.svelte';

	const vm = useFilterBar();
</script>

<svelte:window onresize={vm.updateSize} />
<div
	class="pointer-events-none fixed left-0 top-0 z-20 grid w-full gap-y-2 pb-3
        {isMacos() ? 'sm:justify-end' : ''}
        {isMacos() ? 'right-0' : 'left-0'}
        {modalStore.show ? 'opacity-10 blur-sm' : ''} animate__animated animate__slideInDown
        transition-opacity duration-300"
	style="margin-top: {isMobile() ? mobileStore.statusBarHeight : 8}px;
        grid-template-columns: {vm.state.gridSize};"
	bind:this={vm.element}
>
	<div
		class="grid gap-x-1 pe-2 ps-3 sm:pe-3 sm:ps-3 md:gap-x-3
		{isMacos() ? 'ms-[68px]' : ''}
		{isWindows() || isLinux() ? 'me-[100px] sm:me-0' : ''}
		{isMobile()
			? 'grid-cols-[min-content_1fr_min-content_min-content] sm:grid-cols-[min-content_min-content_1fr]'
			: 'grid-cols-[1fr_min-content_min-content] sm:grid-cols-[min-content_1fr]'}"
	>
		{#if isMobile()}
			<Button
				class="pointer-events-auto grid aspect-square h-9 justify-center rounded sm:p-0"
				onclick={vm.handleMenuButton}
			>
				<div class="w-5">
					<Icon type={IconType.Menu} />
				</div>
			</Button>
		{/if}

		<Input
			class="pointer-events-auto h-9 rounded p-0 sm:hidden"
			icon={IconType.Search}
			placeholder="Search..."
			bind:value={filterStore.search}
		/>

		<Button
			class="pointer-events-auto grid aspect-square h-9 justify-center rounded"
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

		{#if isMobile()}
			<Button
				class="pointer-events-auto grid aspect-square h-9 justify-center rounded sm:hidden sm:p-0"
				onclick={vm.handleQueueButton}
			>
				<div class="w-5">
					<Icon type={IconType.Queue} />
				</div>
			</Button>
		{/if}

		{#if musicStore.listType === 'playlist'}
			<div class="pointer-events-auto hidden h-9 w-full min-w-0 sm:flex sm:justify-start">
				{#if playlistStore.isCreating}
					<div class="grid h-9 w-full grid-cols-[auto_auto] gap-x-2">
						<Button
							class="grid h-9 w-full items-center justify-center gap-x-2 rounded p-[3.5px] sm:p-0 sm:px-2"
							onclick={vm.confirmPlaylistCreation}
						>
							<div class="w-5">
								<Icon type={IconType.Check} />
							</div>
						</Button>
						<Button
							class="grid h-9 w-full items-center justify-center gap-x-2 rounded p-[3.5px] sm:p-0 sm:px-2"
							onclick={vm.cancelPlaylistCreation}
						>
							<div class="w-5">
								<Icon type={IconType.Cancel} />
							</div>
						</Button>
					</div>
				{:else}
					<Button
						class="flex h-9 w-full items-center justify-between gap-x-1 overflow-hidden rounded px-2"
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
				class="pointer-events-auto hidden h-9 w-full sm:block"
				iconStyle="width: {vm.iconSize}px;"
				options={vm.musicListOptions}
				selected={musicStore.listType}
				onchange={vm.handleToggleChange}
			/>
		{/if}
	</div>

	<div class="h-9 px-3 sm:hidden">
		{#if musicStore.listType === 'playlist'}
			<div class="pointer-events-auto flex h-9 w-full min-w-0 justify-end">
				{#if playlistStore.isCreating}
					<div class="grid h-9 w-full grid-cols-[auto_auto] gap-x-2">
						<Button
							class="grid h-9 w-full items-center justify-center gap-x-2 rounded p-[3.5px]"
							onclick={vm.confirmPlaylistCreation}
						>
							<div class="w-5">
								<Icon type={IconType.Check} />
							</div>
						</Button>
						<Button
							class="grid h-9 w-full items-center justify-center gap-x-2 rounded p-[3.5px]"
							onclick={vm.cancelPlaylistCreation}
						>
							<div class="w-5">
								<Icon type={IconType.Cancel} />
							</div>
						</Button>
					</div>
				{:else}
					<Button
						class="flex h-9 w-full items-center justify-between gap-x-1 overflow-hidden rounded px-2"
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
				class="pointer-events-auto w-full"
				iconStyle="width: {vm.iconSize}px;"
				options={vm.musicListOptions}
				selected={musicStore.listType}
				onchange={vm.handleToggleChange}
			/>
		{/if}
	</div>
	<div class="hidden sm:block"></div>
	<div
		class="hidden sm:grid sm:ps-3
		{isMobile() ? '' : 'gap-x-1 sm:grid-cols-[1fr_min-content] md:gap-x-3'}
		{isWindows() || isLinux() ? 'me-[100px]' : ''}"
	>
		<Input
			class="pointer-events-auto h-9 rounded p-0"
			icon={IconType.Search}
			placeholder="Search..."
			bind:value={filterStore.search}
		/>

		{#if isMobile()}
			<Button
				class="pointer-events-auto grid aspect-square h-9 justify-center rounded sm:p-0"
				onclick={vm.handleQueueButton}
			>
				<div class="w-5">
					<Icon type={IconType.Queue} />
				</div>
			</Button>
		{/if}
	</div>
</div>
