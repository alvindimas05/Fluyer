<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import View from '$lib/ui/components/View.svelte';
	import musicStore from '$lib/stores/music.svelte';
	import { MusicListType } from '$lib/features/music/types';
	import folderStore from '$lib/stores/folder.svelte';
	import useCollectionInfo from '$lib/features/collection/viewmodels/useCollectionInfo.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import { isLinux } from '$lib/platform';

	const vm = useCollectionInfo();
	let shouldShow = $derived(
		vm.album || folderStore.currentFolder || musicStore.listType === MusicListType.Playlist
	);
	let isSidebarVisible = $derived(!!sidebarStore.showType);
</script>

{#if shouldShow}
	<View
		class="animate__animated mx-3 mb-2 box-border grid-cols-[auto_max-content] rounded-lg
        px-4 py-2 hover:px-5
        hover:py-3 md:grid
		{isSidebarVisible ? 'animate__fadeOut' : 'animate__fadeIn'}"
		style="animation-duration: {isLinux() ? '350ms' : '500ms'};"
	>
		<div class="grid items-center">
			<div class="overflow-hidden text-sm font-medium md:text-base">
				<p class="animate-scroll-overflow-text overflow-x-hidden whitespace-nowrap">{vm.label}</p>
			</div>
		</div>
		<div class="w-fit">
			<div></div>
			{#await vm.showBackButton then showBackButton}
				<div
					class="mt-2 grid gap-x-2 md:mt-0 md:gap-x-3"
					class:grid-cols-4={showBackButton}
					class:grid-cols-3={!showBackButton}
				>
					{#if showBackButton}
						<button
							class="flex h-6 w-6 items-center justify-center md:h-7 md:w-7"
							onclick={vm.handleBack}
						>
							<Icon type={IconType.AlbumBack} />
						</button>
					{/if}
					<button
						class="flex h-6 w-6 items-center justify-center md:h-7 md:w-7"
						onclick={vm.addMusicListAndPlay}
					>
						<Icon type={IconType.Play} />
					</button>
					<button
						class="flex h-6 w-6 items-center justify-center md:h-7 md:w-7"
						onclick={vm.addMusicList}
					>
						<Icon type={IconType.QueueMusic} />
					</button>
					<button
						class="flex h-6 w-6 items-center justify-center md:h-7 md:w-7"
						onclick={vm.playShuffle}
					>
						<Icon type={IconType.Shuffle} />
					</button>
				</div>
			{/await}
		</div>
	</View>
{:else}
	<div></div>
{/if}
