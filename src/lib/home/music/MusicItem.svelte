<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { filterAlbum, filterSearch } from "$lib/stores/filter";

interface Props {
	music: MusicData;
}

let { music }: Props = $props();

let isValidSearch = $derived.by(() => {
	const search = $filterSearch.toLowerCase();
	const album = $filterAlbum;

	const hasSearch = !!search;
	const hasAlbum = !!album;

	const matchesSearch =
		hasSearch &&
		(music.album?.toLowerCase().includes(search) ||
			music.title?.toLowerCase().includes(search) ||
			music.artist?.toLowerCase().includes(search) ||
			music.albumArtist?.toLowerCase().includes(search));

	const matchesAlbum = hasAlbum && album.name === music.album;

	if (!hasAlbum) {
		return !hasSearch || matchesSearch;
	} else {
		return matchesAlbum && (!hasSearch || matchesSearch);
	}
});

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

async function addMusic() {
	await MusicController.addMusic(music);
}

onMount(checkAlbumImage);
</script>

<div class={`relative text-sm animate__animated animate__fadeIn animate__slow
	${!isValidSearch && "hidden"}`}>
	<div
		class="grid grid-cols-[max-content_auto_max-content] py-2"
	>
		<img
			class="w-12 md:w-14 relative rounded shadow-lg"
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
		<div class="w-12 md:w-14"></div>
	</div>
	<div class="absolute top-0 left-0 py-2 w-full">
		<div class="w-full grid grid-cols-[max-content_auto_max-content] music-item-play animate__animated animate__faster animate__fadeOut">
			<button
					class="w-12 h-12 md:w-14 md:h-14"
					onclick={addMusicAndPlay}
			>
				<div
						class="bg-black bg-opacity-40 grid w-full h-full p-1 justify-items-center items-center rounded"
				><Icon type={IconType.Play}/></div>
			</button>
			<div></div>
			<div class="w-12 h-12 md:w-14 md:h-14 p-2">
				<button class="w-full h-full" onclick={addMusic}><Icon type={IconType.QueuePlaylist} /></button>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	.music-item-play:hover {
		animation-name: fadeIn;
	}
</style>
