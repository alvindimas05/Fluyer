<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";

interface Props {
	music: MusicData;
}

let { music }: Props = $props();

let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null || music.artist == null) return;
	const status = await CoverArt.fromQuery({
		artist: music.artist!,
		title: music.title!,
		album: music.album ?? undefined,
	});
	if (status == CoverArtStatus.Failed) return;
	if (status == CoverArtStatus.Loading) {
		const unlisten = coverArtCaches.subscribe(() => {
			if (setAlbumImageFromCache()) setTimeout(() => unlisten(), 0);
		});
		return;
	}

	setAlbumImageFromCache();
}

function setAlbumImageFromCache() {
	if (albumImage != MusicConfig.defaultAlbumImage) return true;

	const cache = MusicController.getCoverArtCache({
		artist: music.artist!,
		title: music.title!,
		album: music.album ?? undefined,
	});

	if (
		cache == null ||
		(cache.status == CoverArtStatus.Loading && cache.image == null)
	)
		return false;
	if (cache.status == CoverArtStatus.Failed) return true;

	albumImage = MusicController.withBase64(cache.image!);
	return true;
}

async function addMusicAndPlay() {
	music.image = albumImage;
	await MusicController.reset();
	await MusicController.addMusic(music);
	MusicController.play();
}

onMount(checkAlbumImage);
</script>

<div class="relative animate__animated animate__fadeInDown animate__slow">
	<div
		class="grid grid-cols-[max-content_auto] py-2"
	>
		<img
			class="w-12 h-12 lg:w-14 lg:h-14 xl:w-16 xl:h-16 relative rounded"
			src={albumImage}
			alt="Album"
		/>
		<div class="ms-3 overflow-hidden">
			<p
				class="font-medium whitespace-nowrap overflow-hidden animate-scroll-overflow-text"
			>
				{music.title}
			</p>
			<p
				class="text-opacity-background-80 whitespace-nowrap overflow-hidden animate-scroll-overflow-text"
			>
				{MusicController.getFullArtistFromMusic(music)}
			</p>
		</div>
	</div>
	<div class="absolute top-0 left-0 py-2 w-full music-item-play animate__animated animate__faster animate__fadeOut">
		<button
			class="w-12 h-12 lg:w-14 lg:h-14 xl:w-16 xl:h-16"
			onclick={addMusicAndPlay}
		>
			<div
				class="bg-black bg-opacity-40 grid w-full h-full justify-items-center items-center rounded"
			>
				<img
					class="w-10 h-10 lg:w-12 lg:h-12 invert"
					src={MusicConfig.defaultPlayButton}
					alt="Play"
				/>
			</div>
		</button>
	</div>
</div>

<style lang="scss">
	.music-item-play:hover {
		animation-name: fadeIn;
	}
</style>
