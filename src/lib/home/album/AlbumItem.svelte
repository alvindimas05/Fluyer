<script lang="ts">
import SpotifyApi from "$lib/api/spotify";
import { onMount } from "svelte";
import type { MusicData } from "../music/types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";

interface Props {
	musicList: MusicData[];
	index: number;
}

let { musicList, index }: Props = $props();
let music = musicList[0];

const animationDelay = 200;
let animationClasses = $state("hidden");

const spotifyApi = new SpotifyApi();
let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null) return;
	const spotifyMusic = await spotifyApi.searchMusic(music);
	if (spotifyMusic == null) return;
	albumImage = spotifyMusic?.imageUrl;
	musicList = musicList.map((m) => {
		m.image = albumImage;
		return m;
	});
}

async function sortMusicList() {
    const hasTrackNumber = musicList[0].trackNumber != null;
    musicList = musicList.sort((a, b) => {
        if (hasTrackNumber) {
            if (a.trackNumber?.includes("/") || b.trackNumber?.includes("/")) {
                a.trackNumber = a.trackNumber!.split("/")[0];
                b.trackNumber = b.trackNumber!.split("/")[0];
            }
            return +a.trackNumber! - +b.trackNumber!;
        } else {
            return a.filename.localeCompare(b.filename);
        }
    });
}

async function addMusicListAndPlay() {
	music.image = albumImage;
	await MusicController.clear();
	await MusicController.addMusicList(musicList);
	MusicController.play(true);
}

onMount(() => {
	sortMusicList();
	checkAlbumImage();
});

setTimeout(
	() => (animationClasses = "animate__animated animate__fadeInDown"),
	animationDelay * index,
);
</script>

<div class={`px-3 pb-6 lg:pt-6 text-white row-[1] col-auto ${animationClasses}`}>
	<div class="relative w-full">
		<div
			class="album-item-actions w-full h-full absolute rounded-lg bg-gradient-to-b from-transparent to-black/75
			animate__animated animate__faster animate__fadeOut"
		>
			<button
				class="w-12 h-12 absolute bottom-0 left-0 ms-3 mb-3"
				onclick={addMusicListAndPlay}
			>
				<img
					class="invert"
					src={MusicConfig.defaultPlayButton}
					alt="Play"
				/></button
			>
		</div>
		<img class="rounded-lg w-full" src={albumImage} alt="Album" />
	</div>
	<p class="font-medium text-xl mt-2">{music.album}</p>
	<p class="text-lg text-gray-200">
		{MusicController.getFullArtistFromMusic(music)}
	</p>
</div>

<style lang="scss">
	.album-item-actions:hover {
		animation-name: fadeIn;
	}
</style>
