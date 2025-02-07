<script lang="ts">
import { onMount } from "svelte";
import type { MusicData } from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import CoverArt, { CoverArtStatus } from "$lib/handlers/coverart";
    import { isAndroid, isIos, isMacos } from "$lib/platform";
    import { coverArtCaches } from "$lib/stores/coverart";

interface Props {
	music: MusicData;
}

let { music }: Props = $props();

let albumImage = $state(MusicController.getAlbumImageFromMusic(music));

async function checkAlbumImage() {
	if (music.image !== null || music.artist == null) return;
	// const spotifyMusic = await spotifyApi.searchMusic(music);
	// if (spotifyMusic == null) return;
	// albumImage = spotifyMusic?.imageUrl;
	const status = await CoverArt.fromQuery({
	    artist: music.artist!,
		title: music.title!,
		album: music.album ?? undefined
	});
	if (status == CoverArtStatus.Failed) return;
	if (status == CoverArtStatus.Loading){
	    // Note: Blame Webkit for this shit. Always gives error "Uninitialized variable" when trying to call unlisten :)
		// Note: I have no idea why this happens on Android as well.
        if(isMacos() || isIos() || isAndroid()){
            coverArtCaches.subscribe(() => {
                setAlbumImageFromCache()
            });
        } else {
            const unsub = coverArtCaches.subscribe(() => {
                if(setAlbumImageFromCache()) unsub()
            });
        }
        return;
	}
	
	setAlbumImageFromCache();
}

function setAlbumImageFromCache(){
    if(albumImage != MusicConfig.defaultAlbumImage) return true;
    
	const cache = MusicController.getCoverArtCache({ artist: music.artist!, title: music.title!, album: music.album ?? undefined });
	if (cache == null) return false;
	if (cache.status == CoverArtStatus.Failed || cache.image == null) return true;
   
	albumImage = MusicController.withBase64(cache.image!);
	return true;
}

async function addMusicAndPlay() {
	music.image = albumImage;
	const previousMusic = MusicController.currentMusic();
	await MusicController.addMusic(music);
	if (
		previousMusic === null ||
		(previousMusic !== null && MusicController.isCurrentMusicFinished())
	)
		MusicController.play();
}

// onMount(checkAlbumImage);
</script>

<div
    class="grid grid-cols-[max-content_auto] py-2 animate__animated animate__fadeInDown animate__slow"
>
    <button class="w-12 h-12 lg:w-14 lg:h-14 xl:w-16 xl:h-16 relative" onclick={addMusicAndPlay}>
        <div
            class="music-item-play bg-black bg-opacity-40 absolute grid w-full h-full
        justify-items-center items-center animate__animated animate__faster animate__fadeOut rounded"
        >
            <img
                class="w-10 h-10 lg:w-12 lg:h-12 invert"
                src={MusicConfig.defaultPlayButton}
                alt="Play"
            />
        </div>
        <img class="w-12 lg:w-16 rounded" src={albumImage} alt="Album" />
    </button>
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-200">
            {MusicController.getFullArtistFromMusic(music)}
        </p>
    </div>
</div>

<style lang="scss">
    .music-item-play:hover {
        animation-name: fadeIn;
    }
</style>
