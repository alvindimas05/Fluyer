<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import {
	musicCurrent,
	musicIsPlaying,
	musicProgressValue,
	musicVolume,
} from "$lib/stores/music";
import { goto } from "$app/navigation";
import { mobileNavigationBarHeight } from "$lib/stores/mobile";
    import { backgroundIsLight } from "$lib/stores/background";
    import { isAndroid } from "$lib/platform";

// Based on Rust Rodio fade effect (Please check player.rs)
let pauseDelay = isAndroid() ? 0 : 400;
let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $state(MusicConfig.defaultAlbumImage);

let isPlaying = $state(MusicController.isPlaying());
let progressPercentage = $state(MusicController.progressPercentage());
let volumePercentage = $state(MusicController.volumePercentage());

musicProgressValue.subscribe(updateStates);
musicCurrent.subscribe(updateStates);
musicIsPlaying.subscribe(updateStates);
musicVolume.subscribe(() => {
	volumePercentage = MusicController.volumePercentage();
});

function handleButtonPlayPause() {
	if (
		MusicController.currentMusic() === null ||
		MusicController.isProgressValueEnd()
	)
		return;

	if (MusicController.isPlaying()) {
		MusicController.setIsPlaying(false);
		setTimeout(MusicController.pause, pauseDelay);
	} else MusicController.play();

	updateStates();
}

function handleButtonNext() {
	MusicController.nextMusic();
}

function onPlayerBarChange() {
	if (MusicController.isProgressValueEnd()) {
		MusicController.setProgressValue(0);
		return;
	}

	if (MusicController.isProgressValueEnd()) {
		MusicController.addMusic(MusicController.currentMusic()!);
	}

	MusicController.sendCommandSetPosition(
		MusicController.realProgressDuration(),
	);
}

function updateStates() {
	isPlaying = MusicController.isPlaying();
	progressPercentage = MusicController.progressPercentage();

	let music = MusicController.currentMusic();
	if (music == null) return;

	title = music.title!;
	artist = MusicController.getFullArtistFromMusic(music);
	albumImage = MusicController.currentMusicAlbumImage();
}

async function onKeyDown(
	e: KeyboardEvent & {
		currentTarget: EventTarget & Document;
	},
) {
	if (e.key === " ") handleButtonPlayPause();
}

function redirectToPlay() {
	goto("/play");
}

function handleVolumeButton() {
	MusicController.setVolume(MusicController.volume() > 0 ? 0 : 1);
}
</script>

<svelte:document onkeydown={onKeyDown} />

<div
    class="fixed left-0 bottom-0 z-10 w-full bg-gray-700 bg-opacity-30 backdrop-blur-md text-white animate__animated animate__slideInUp animate__slow"
    style={`padding-bottom: ${$mobileNavigationBarHeight}px`}
>
    <input
        class={`w-full absolute music-progress-bar music-progress-bar-${$backgroundIsLight ? "light" : "dark"}`}
        type="range"
        style={`--progress-width: ${progressPercentage}%`}
        bind:value={$musicProgressValue}
        min={MusicConfig.min}
        max={MusicConfig.max}
        step={MusicConfig.step}
        onchange={onPlayerBarChange}
    />
    <div class="p-3 mt-1">
        <div class="grid grid-cols-[auto_min-content] lg:grid-cols-3">
            <div class="flex items-center">
                <!-- TODO: Button Previous Functionality -->
                <button class="w-8 md:w-10 tb:w-10 lg:w-10 invert mx-2"
                    ><img
                        src={MusicConfig.defaultPreviousButton}
                        alt="Icon Previous"
                    /></button
                >
                <button
                    class="w-8 md:w-10 tb:w-10 lg:w-10 invert mx-2"
                    onclick={handleButtonPlayPause}
                    ><img
                        src={isPlaying
                            ? MusicConfig.defaultPauseButton
                            : MusicConfig.defaultPlayButton}
                        alt="Icon Play"
                    /></button
                >
                <button
                    class="w-8 md:w-10 tb:w-10 lg:w-10 invert mx-2"
                    onclick={handleButtonNext}
                    ><img
                        src={MusicConfig.defaultNextButton}
                        alt="Icon Next"
                    /></button
                >
            </div>
            <div
                class="ms-2 lg:ms-0 lg:flex items-center justify-center order-first lg:order-none"
            >
                <div
                    class="grid grid-cols-[2.5rem_auto] md:grid-cols-[3rem_auto]"
                >
                    <button onclick={redirectToPlay}>
                        <img class="w-12 lg:w-16 rounded" src={albumImage} alt="Album" />
                    </button>
                    <!-- FIXME: Overflow Text Animation -->
                    <div class="ms-3 overflow-hidden">
                        <p class="font-medium whitespace-nowrap overflow-hidden">
                            {title}
                        </p>
                        <p
                            class="text-gray-200 whitespace-nowrap overflow-hidden"
                        >
                            {artist}
                        </p>
                    </div>
                </div>
            </div>
            <div class="hidden lg:grid justify-end">
                <div class="grid grid-cols-[auto_auto] items-center gap-3">
                    <button onclick={handleVolumeButton}>
                        <img class="invert w-5" src={volumePercentage > 0 ? MusicConfig.defaultSpeakerButton : MusicConfig.defaultMuteButton}
                            alt="Volume"/>
                    </button>
                    <input
                        class={`w-24 volume-progress-bar volume-progress-bar-${$backgroundIsLight ? "light" : "dark"}`}
                        type="range"
                        style={`--progress-width: ${volumePercentage}%`}
                        bind:value={$musicVolume}
                        min={MusicConfig.vmin}
                        max={MusicConfig.vmax}
                        step={MusicConfig.vstep}>
                </div>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
</style>