<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import {
	musicCurrent,
	musicIsPlaying,
	musicProgressValue,
} from "$lib/stores/music";
import { goto } from "$app/navigation";
import { mobileNavigationBarHeight } from "$lib/stores/mobile";

// Based on Rust Rodio fade effect (Please check player.rs)
let pauseDelay = 400;
let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $state(MusicConfig.defaultAlbumImage);

let isPlaying = $state(MusicController.isPlaying());
let progressPercentage = $state(MusicController.progressPercentage());
let navigationBarHeight = $state(0);

musicProgressValue.subscribe(updateStates);
musicCurrent.subscribe(updateStates);
musicIsPlaying.subscribe(updateStates);

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
</script>

<svelte:document onkeydown={onKeyDown} />

<div
    class="fixed left-0 bottom-0 z-10 w-full bg-gray-700 bg-opacity-30 backdrop-blur-md text-white animate__animated animate__slideInUp animate__slow"
    style={`padding-bottom: ${$mobileNavigationBarHeight}px`}
>
    <input
        id="music-progress-bar"
        class="w-full absolute"
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
                    <div class="ms-3 overflow-hidden">
                        <p class="font-medium">
                            {title}
                        </p>
                        <p
                            class="text-gray-200 whitespace-nowrap overflow-ellipsis"
                        >
                            {artist}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
    #music-progress-bar {
        @apply cursor-pointer outline-0;
        appearance: none;
        
        &::-webkit-slider-runnable-track {
            @apply h-[.2rem] rounded;
            background: linear-gradient(to right, #fff var(--progress-width), #9ca3af var(--progress-width));
        }
        &::-webkit-slider-thumb {
            @apply mt-[.2rem] invisible;
        }
    }    
</style>