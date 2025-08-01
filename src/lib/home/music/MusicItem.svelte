<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtCaches } from "$lib/stores/coverart";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { musicList } from "$lib/stores/music";
import {isDesktop} from "$lib/platform";

interface Props {
	music: MusicData;
}

let { music }: Props = $props();

let albumImage = $derived(MusicController.getAlbumImageFromMusic(music));
let infoLabel = $derived.by(() => {
	const duration = MusicController.parseMilisecondsIntoText(music.duration);
	let audioResolution: any = [music.bitsPerSample ?? 0,
		MusicController.parseSampleRateIntoText(music.sampleRate)]
		.filter((v) => !!v);
	if(audioResolution.length) audioResolution = audioResolution.join(MusicConfig.separatorAudio);
	else return duration;

	return [audioResolution, duration].join(` ${MusicConfig.separator} `);
});

async function addMusicAndPlay() {
	await MusicController.resetAndAddMusic(music);
	MusicController.play();
}

async function addMusic() {
	await MusicController.addMusic(music);
}
</script>

<div class="relative text-sm md:text-base">
	<div
		class="grid grid-cols-[max-content_auto_max-content] py-2"
	>
		{#await albumImage}
			<div class="w-12 md:w-14 relative aspect-square"></div>
		{:then image}
			<img
				class="w-12 md:w-14 relative rounded {isDesktop() && 'animate__animated animate__fadeIn'}"
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
				class="text-opacity-background-90 whitespace-nowrap overflow-hidden
				text-xs animate-scroll-overflow-text"
			>
				{music.album ? `${music.album} ${MusicConfig.separatorAlbum} ` : ''}
				{MusicController.getFullArtistFromMusic(music)}
			</p>
			<p class="text-xs text-opacity-background-90">{infoLabel}</p>
		</div>
		<div class="w-12 md:w-14"></div>
	</div>
	<div class="absolute top-0 left-0 py-2 w-full">
		<div class="w-full grid grid-cols-[max-content_auto_max-content] music-item-play">
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
	.music-item-play {
		opacity: 0;

		&:hover {
			animation-name: fadeIn;
			animation-duration: 0.5s;
			animation-fill-mode: forwards;
		}
	}
</style>
