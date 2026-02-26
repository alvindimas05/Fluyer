<script lang="ts">
	import playlistStore from '$lib/stores/playlist.svelte';
	import TauriPlaylistAPI from '$lib/tauri/TauriPlaylistAPI';
	import type { PlaylistData } from '$lib/features/music/types';
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import Button from '$lib/ui/components/Button.svelte';

	let title = $state('');
	let artist = $state('');
	let imageSource = $state<'none' | 'music' | 'upload'>('none');
	let selectedMusicPath = $state('');
	let uploadedImagePath = $state('');
	let isSubmitting = $state(false);

	async function handleImageUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		const buffer = await file.arrayBuffer();
		const imageData = Array.from(new Uint8Array(buffer));
		const filename = `playlist_${Date.now()}_${file.name}`;

		try {
			uploadedImagePath = await TauriPlaylistAPI.saveImage(imageData, filename);
		} catch (e) {
			console.error('Failed to save image:', e);
		}
	}

	function getImageValue(): string | undefined {
		switch (imageSource) {
			case 'music':
				return selectedMusicPath || undefined;
			case 'upload':
				return uploadedImagePath || undefined;
			default:
				return undefined;
		}
	}

	async function handleSubmit() {
		if (!title.trim()) return;
		isSubmitting = true;

		const playlist: PlaylistData = {
			name: title.trim(),
			title: title.trim(),
			artist: artist.trim() || undefined,
			image: getImageValue(),
			paths: playlistStore.selectedPaths
		};

		try {
			await TauriPlaylistAPI.create(playlist);
			// Refresh playlist list
			playlistStore.list = await TauriPlaylistAPI.getAll();
			close();
		} catch (e) {
			console.error('Failed to create playlist:', e);
		} finally {
			isSubmitting = false;
		}
	}

	function close() {
		playlistStore.showCreateModal = false;
		playlistStore.isCreating = false;
		playlistStore.selectedPaths = [];
		title = '';
		artist = '';
		imageSource = 'none';
		selectedMusicPath = '';
		uploadedImagePath = '';
	}
</script>

{#if playlistStore.showCreateModal}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
	>
		<div
			class="mx-4 w-full max-w-md rounded-xl border border-white/10 bg-neutral-900/95 p-6 shadow-2xl"
		>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-lg font-semibold text-white">Create Playlist</h2>
				<Button class="grid h-6 w-6 justify-center rounded" onclick={close}>
					<Icon type={IconType.Close} />
				</Button>
			</div>

			<div class="space-y-4">
				<!-- Title -->
				<div>
					<label class="mb-1 block text-sm text-white/70" for="playlist-title">Title</label>
					<input
						id="playlist-title"
						type="text"
						bind:value={title}
						placeholder="Playlist name..."
						class="w-full rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white
							placeholder:text-white/30 focus:border-white/30 focus:outline-none"
					/>
				</div>

				<!-- Artist -->
				<div>
					<label class="mb-1 block text-sm text-white/70" for="playlist-artist">Artist</label>
					<input
						id="playlist-artist"
						type="text"
						bind:value={artist}
						placeholder="Artist name..."
						class="w-full rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white
							placeholder:text-white/30 focus:border-white/30 focus:outline-none"
					/>
				</div>

				<!-- Image -->
				<div>
					<label class="mb-2 block text-sm text-white/70">Image</label>
					<div class="space-y-2">
						<label class="flex cursor-pointer items-center gap-2 text-sm text-white/80">
							<input type="radio" bind:group={imageSource} value="none" class="accent-white" />
							Use default
						</label>
						<label class="flex cursor-pointer items-center gap-2 text-sm text-white/80">
							<input type="radio" bind:group={imageSource} value="music" class="accent-white" />
							From music cover
						</label>
						{#if imageSource === 'music'}
							<select
								bind:value={selectedMusicPath}
								class="w-full rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white
									focus:border-white/30 focus:outline-none"
							>
								<option value="">Select a track...</option>
								{#each playlistStore.selectedPaths as path}
									<option value={path}>{path.split('/').pop() || path}</option>
								{/each}
							</select>
						{/if}
						<label class="flex cursor-pointer items-center gap-2 text-sm text-white/80">
							<input type="radio" bind:group={imageSource} value="upload" class="accent-white" />
							Upload image
						</label>
						{#if imageSource === 'upload'}
							<input
								type="file"
								accept="image/*"
								onchange={handleImageUpload}
								class="w-full text-sm text-white/60 file:mr-3 file:rounded-lg file:border-0
									file:bg-white/10 file:px-3 file:py-1.5 file:text-sm file:text-white
									hover:file:bg-white/20"
							/>
							{#if uploadedImagePath}
								<p class="text-xs text-green-400">Image saved ✓</p>
							{/if}
						{/if}
					</div>
				</div>

				<!-- Track count -->
				<p class="text-sm text-white/50">
					{playlistStore.selectedPaths.length} track{playlistStore.selectedPaths.length !== 1
						? 's'
						: ''} selected
				</p>

				<!-- Actions -->
				<div class="flex justify-end gap-2 pt-2">
					<button
						class="rounded-lg border border-white/10 px-4 py-2 text-sm text-white/70
							transition-colors hover:bg-white/5"
						onclick={close}
					>
						Cancel
					</button>
					<button
						class="rounded-lg bg-white/15 px-4 py-2 text-sm font-medium text-white
							transition-colors hover:bg-white/25 disabled:opacity-50"
						onclick={handleSubmit}
						disabled={!title.trim() || isSubmitting}
					>
						{isSubmitting ? 'Creating...' : 'Create'}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
