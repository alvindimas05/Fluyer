<script lang="ts">
import {type FolderData, type MusicData, MusicListType} from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { isDesktop } from "$lib/platform";
import FolderController from "$lib/controllers/FolderController";
import {musicListType} from "$lib/stores/music";
import {folderCurrent} from "$lib/stores/folder";

interface Props {
	music: MusicData;
	folder?: FolderData;
}

let { music, folder }: Props = $props();

let albumImage = $derived.by(() => folder ? FolderController.getImageFromPath(folder.path)
	: MusicController.getAlbumImageFromMusic(music));
let titleLabel = $derived.by(() => {
	if(folder){
		if($folderCurrent){
			return folder.path.split(FolderController.pathSeparator).pop();
		}
		return folder.path;
	} else {
		if($musicListType === MusicListType.Folder){
			return music.filename;
		}
		return music.title;
	}
});
let mediumLabel = $derived.by(() => {
	if(folder){
		return 'Folder';
	} else {
		return `${music.album ? `${music.album} ${MusicConfig.separatorAlbum} ` : ''} ${MusicController.getFullArtistFromMusic(music)}`;
	}
});
let smallLabel = $derived.by(() => {
	if(folder){
		const folderMusic = FolderController.getMusicListFromFolder(folder);
		const totalDuration = folderMusic.reduce((acc, music) => acc + music.duration, 0);

		return `${folderMusic.length} ${MusicConfig.separator} ${MusicController.parseMilisecondsIntoText(totalDuration)}`;
	} else {
		const duration = MusicController.parseMilisecondsIntoText(music.duration);
		let audioResolution: any = [
			music.bitsPerSample ? `${music.bitsPerSample}-bit` : 0,
			MusicController.parseSampleRateIntoText(music.sampleRate),
		].filter((v) => !!v);
		if (audioResolution.length)
			audioResolution = audioResolution.join(MusicConfig.separatorAudio);
		else return duration;

		return [audioResolution, duration].join(` ${MusicConfig.separator} `);
	}
});

async function addMusicAndPlay() {
	if (music){
		await MusicController.resetAndAddMusic(music);
	} else {
		await MusicController.resetAndAddMusicList(FolderController.getMusicListFromFolder(folder!!));
	}
	MusicController.play();
}

async function addMusic() {
	if (music){
		await MusicController.addMusic(music);
	} else {
		await MusicController.resetAndAddMusicList(FolderController.getMusicListFromFolder(folder!!));
	}
}

async function selectFolder(){
	if (!folder) return;
	await FolderController.setFolder(folder);
}
</script>

<div class="relative text-sm md:text-base">
	<div
		class="grid grid-cols-[max-content_auto] py-2"
	>
		{#await albumImage}
			<div class="w-12 h-12 md:w-14 md:h-14 relative aspect-square"></div>
		{:then image}
			{#if image}
				<img
					class="w-12 h-12 md:w-14 md:h-14 relative rounded {isDesktop() && 'animate__animated animate__fadeIn'}"
					src={image}
					alt="Album"
				/>
			{:else}
				<div class="w-12 h-12 md:w-14 md:h-14 relative aspect-square">
					<Icon type={IconType.Folder} />
				</div>
			{/if}
		{/await}
		<div class="ms-3 overflow-hidden">
			<p
				class="font-medium text-sm/[14px] md:text-sm whitespace-nowrap overflow-hidden animate-scroll-overflow-text"
			>{titleLabel}</p>
			<p class="text-opacity-background-90 whitespace-nowrap overflow-hidden
				text-xs/[14px] pt-[4px] md:text-xs md:pt-0 animate-scroll-overflow-text">{mediumLabel}</p>
			<p class="text-xs/[14px] md:text-xs mt-[2px] text-opacity-background-90">{smallLabel}</p>
		</div>
	</div>
	<div class="absolute top-0 left-0 py-2 w-full">
		<div class="w-full grid grid-cols-[max-content_auto_max-content] music-item-play">
			<button
					class="w-12 h-12 md:w-14 md:h-14"
					onclick={addMusicAndPlay}
			>
				<div
						class="bg-black bg-opacity-40 grid p-1 box-border justify-items-center items-center rounded"
				><Icon type={IconType.Play}/></div>
			</button>
			<div class="{folder ? 'cursor-pointer' : 'cursor-default'}" onclick={selectFolder}></div>
			<div class="w-12 h-12 md:w-14 md:h-14 ps-4">
				<button class="w-full h-full aspect-square" onclick={addMusic}>
					<Icon type={IconType.QueuePlaylist} />
				</button>
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
