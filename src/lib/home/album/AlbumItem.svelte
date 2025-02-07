<script lang="ts">
import SpotifyApi from "$lib/api/spotify";
import { onMount } from "svelte";
import type { MusicData } from "../music/types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import MusicBrainzApi from "$lib/api/musicbrainz";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
import { coverArtAlbumCaches } from "$lib/stores/coverart";
    import { isIos, isMacos } from "$lib/platform";

interface Props {
	musicList: MusicData[];
	index: number;
}

let { musicList, index }: Props = $props();
let music = musicList[0];

const animationDelay = 200;
let animationClasses = $state("hidden");
let isSorted = false;

// const spotifyApi = new SpotifyApi();
let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null) return;
	// const spotifyMusic = await spotifyApi.searchMusic(music);
	// if (spotifyMusic == null) return;
	// albumImage = spotifyMusic?.imageUrl;
	const status = await CoverArt.fromAlbum(music.album!);
	if (status == CoverArtStatus.Failed) return;
	if (status == CoverArtStatus.Loading){
        // Note: Blame Webkit for this shit. Always gives error "Uninitialized variable" when trying to call unlisten :)
        if(isMacos() || isIos()){
            coverArtAlbumCaches.subscribe(() => {
                setAlbumImageFromCache()
            });
        } else {
            let unlisten = coverArtAlbumCaches.subscribe(() => {
                if(setAlbumImageFromCache()) unlisten()
            });
        }
        return;
	}
	
	setAlbumImageFromCache();
}

function setAlbumImageFromCache(){
	const cache = MusicController.getCoverArtAlbumCache(music.album!);
	if (cache == null) return false;
	if (cache.status == CoverArtStatus.Failed || cache.image == null) return true;
   
	albumImage = MusicController.withBase64(cache.image!);
	musicList = musicList.map((m) => {
		m.image = MusicController.withBase64(cache.image!);
		return m;
	});
	return true;
}

async function sortMusicList() {
	const hasTrackNumber = music.trackNumber != null;
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
	if (!isSorted) {
		isSorted = true;
		sortMusicList();
	}
	await MusicController.clear();
	await MusicController.addMusicList(musicList);
	MusicController.play(true);
}

onMount(checkAlbumImage);

setTimeout(
	() => (animationClasses = "animate__animated animate__fadeInDown"),
	animationDelay * index,
);
</script>

<div
    class={`px-3 pb-6 text-white row-[1] col-auto ${animationClasses}`}
>
    <div class="relative w-full">
        <div
            class="album-item-actions w-full h-full absolute rounded-lg bg-gradient-to-b from-transparent to-black/50
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
