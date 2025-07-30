<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { musicList } from "$lib/stores/music";

interface Props {
	music: MusicData;
}

let { music }: Props = $props();

let albumImage = $derived(MusicController.getAlbumImageFromMusic(music));

async function addMusicAndPlay() {
	await MusicController.resetAndAddMusic(music);
	MusicController.play();
}

async function addMusic() {
	await MusicController.addMusic(music);
}
</script>

<div class="relative text-sm md:text-base animate__animated animate__fadeIn">
	<div
		class="grid grid-cols-[max-content_auto_max-content] py-2"
	>
		{#await albumImage}
			<div class="w-12 md:w-14 relative aspect-square"></div>
		{:then image}
			<img
				class="w-12 md:w-14 relative rounded shadow-lg animate__animated animate__fadeIn"
				src={image}
				alt="Album"
			/>
		{/await}
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
