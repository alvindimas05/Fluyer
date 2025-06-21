<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import {
	musicCurrentIndex,
	musicIsPlaying,
	musicProgressValue,
	musicVolume,
} from "$lib/stores/music";
import { mobileNavigationBarHeight } from "$lib/stores/mobile";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";

let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $state(MusicConfig.defaultAlbumImage);

let isPlaying = $state(MusicController.isPlaying());
let progressPercentage = $state(MusicController.progressPercentage());
let volumePercentage = $state(MusicController.volumePercentage());

musicProgressValue.subscribe(updateStates);
musicCurrentIndex.subscribe(updateStates);
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
		MusicController.pause();
	} else MusicController.play();

	updateStates();
}

function handleButtonPrevious() {
	MusicController.previousMusic();
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
	PageController.goto(PageRoutes.PLAY);
}

function handleVolumeButton() {
	MusicController.setVolume(MusicController.volume() > 0 ? 0 : 1);
}
</script>

<svelte:document onkeydown={onKeyDown} />

<div
    class={`fixed left-0 bottom-0 z-10 w-full bg-gray-700 bg-opacity-30 text-white backdrop-blur-md`}
    style={`padding-bottom: ${$mobileNavigationBarHeight}px`}
>
    <div class="relative">
        <input
            class={`w-full absolute music-progress-bar`}
            type="range"
            style={`--progress-width: ${progressPercentage}%`}
            bind:value={$musicProgressValue}
            min={MusicConfig.min}
            max={MusicConfig.max}
            step={MusicConfig.step}
            onchange={onPlayerBarChange}
        />
        <input
            class={`w-full absolute music-progress-bar-end`}
            type="range"
            style={`--progress-width: ${progressPercentage}%`}
            bind:value={$musicProgressValue}
            min={MusicConfig.min}
            max={MusicConfig.max}
            step={MusicConfig.step}
            onchange={onPlayerBarChange}
        />
    </div>
    <div class="p-3 mt-1">
        <div class="grid grid-cols-[auto_min-content] lg:grid-cols-3">
            <div class="flex items-center md:gap-2">
                <button
                    class="w-10 md:w-12 lg:w-12"
                    onclick={handleButtonPrevious}
                    ><Icon type={IconType.Previous} /></button
                >
                <button
                    class="w-10 md:w-12 lg:w-12"
                    onclick={handleButtonPlayPause}
                    >
                    {#if isPlaying}
                        <Icon type={IconType.Pause} />
                    {:else}
                        <Icon type={IconType.Play} />
                    {/if}
                    </button
                >
                <button
                    class="w-10 md:w-12 lg:w-12"
                    onclick={handleButtonNext}
                    ><Icon type={IconType.Next} /></button
                >
            </div>
            <div
                class="ms-2 lg:ms-0  lg:flex items-center justify-center
                order-first  lg:order-none
                text-sm md:text-base  "
            >
                <div
                    class="grid grid-cols-[2.5rem_auto] md:grid-cols-[3rem_auto]
                     "
                >
                    <button onclick={redirectToPlay}>
                        <img
                            class="rounded"
                            src={albumImage}
                            alt="Album"
                        />
                    </button>
                    <div class="ms-3 overflow-hidden">
                        <!-- Note: Idk why the title scroll doesn't work without sacrificing first element -->
                        <p class="animate-scroll-overflow-text"></p>
                        <p
                            class="font-medium whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text"
                        >
                            {title}
                        </p>
                        <p
                            class="text-opacity-80 whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text"
                        >
                            {artist}
                        </p>
                    </div>
                </div>
            </div>
            <div class="hidden  lg:grid justify-end">
                <div class="grid grid-cols-[auto_auto] items-center gap-3">
                    <button class="w-5" onclick={handleVolumeButton}>
                        {#if volumePercentage > 0}
                            <Icon type={IconType.Speaker} />
                        {:else}
                            <Icon type={IconType.Mute} />
                        {/if}
                    </button>
                    <div class="relative w-24">
                        <input
                            class={`absolute w-24 volume-progress-bar-end`}
                            type="range"
                            style={`--progress-width: ${volumePercentage}%`}
                            min={MusicConfig.vmin}
                            max={MusicConfig.vmax}
                            step={MusicConfig.vstep}
                        />
                        <input
                            class={`absolute w-24 volume-progress-bar`}
                            type="range"
                            style={`--progress-width: ${volumePercentage}%`}
                            bind:value={$musicVolume}
                            min={MusicConfig.vmin}
                            max={MusicConfig.vmax}
                            step={MusicConfig.vstep}
                        />
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
    .playerbar-in {
        animation:
            blurIn 4s forwards,
            slideInUp 2s forwards;
    }
    .playerbar-out {
        @apply backdrop-blur-md;
        animation: slideOutDown 0.5s forwards;
    }
    @keyframes blurIn {
        25% {
            @apply backdrop-blur-none;
        }
        100% {
            @apply backdrop-blur-md;
        }
    }
</style>
