<script lang="ts">
import {
	musicCurrentIndex,
	musicIsPlaying,
	musicProgressValue,
	musicVolume,
} from "$lib/stores/music";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import type MusicLyric from "$lib/home/music/lyric";
import { onMount } from "svelte";
import LrcLib from "$lib/api/lrclib";
import {
	mobileNavigationBarHeight,
	mobileStatusBarHeight,
} from "$lib/stores/mobile";
import { isAndroid } from "$lib/platform";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import { pageHomePlayerBarShow } from "$lib/stores/page";

// Based on Rust Rodio fade effect (Please check player.rs)
let pauseDelay = isAndroid() ? 0 : 400;
let music = $state(MusicController.currentMusic());
let progressPercentage = $state(MusicController.progressPercentage());
let progressDurationText = $state(MusicController.progressDurationText());
let progressDurationNegativeText = $state(
	MusicController.progressDurationText(true),
);
let albumImage = $state(MusicConfig.defaultAlbumImage);

let lyrics: MusicLyric[] = $state([]);
let selectedLyricIndex = $state(0);
let volumePercentage = $state(MusicController.volumePercentage());

musicProgressValue.subscribe(() => {
	progressPercentage = MusicController.progressPercentage();
	progressDurationText = MusicController.progressDurationText();
	progressDurationNegativeText = MusicController.progressDurationText(true);

	resetSelectedLyricIndex();
});
musicCurrentIndex.subscribe(() => {
	music = MusicController.currentMusic();
	albumImage = MusicController.currentMusicAlbumImage();
	resetLyrics();
});

function handleButtonPlayPause() {
	if (
		MusicController.currentMusic() === null ||
		MusicController.isProgressValueEnd()
	) {
		return;
	}

	if (MusicController.isPlaying()) {
		MusicController.setIsPlaying(false);
		setTimeout(MusicController.pause, pauseDelay);
	} else MusicController.play();
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

function handleButtonBack() {
	$pageHomePlayerBarShow = true;
	PageController.goto(PageRoutes.HOME);
}

async function onKeyDown(
	e: KeyboardEvent & {
		currentTarget: EventTarget & Document;
	},
) {
	if (e.key === " ") handleButtonPlayPause();
}

async function resetLyrics() {
	selectedLyricIndex = 0;

	if (MusicController.currentMusic() == null) return;
	const resLyrics = await LrcLib.getLyrics(MusicController.currentMusic()!);
	if (resLyrics == null){
        lyrics = [];
        return;
    }
	lyrics = resLyrics;
}

function resetSelectedLyricIndex() {
	if (lyrics.length < 1) return;

	if (MusicController.progressDuration() < lyrics[0].duration) {
		scrollToSelectedLyric();
		return;
	}
	// Note: Using for loop since it's the fastest. Just in case though :)
	for (var i = 0; i < lyrics.length; i++) {
		if (MusicController.progressDuration() < lyrics[i].duration) {
			selectedLyricIndex = i - 1;
			scrollToSelectedLyric();
			return;
		}
	}
	selectedLyricIndex = lyrics.length - 1;
	scrollToSelectedLyric();
}

function scrollToSelectedLyric() {
	document.getElementById("selected-lyric")?.scrollIntoView({
		block: window.innerWidth > 768 ? "center" : "start",
		behavior: "smooth",
	});
}

musicVolume.subscribe(() => {
	volumePercentage = MusicController.volumePercentage();
});
</script>

<svelte:document onkeydown={onKeyDown} />

<div
    id="root-play"
    class={`w-full h-full grid mx-auto max-w-[35rem] md:max-w-none md:gap-y-0
    ${lyrics.length > 1 ? "md:grid-cols-[40%_55%]" : "md:grid-cols-[45%] justify-center root-nolyrics"} md:pt-0`}
    style={`--mobile-status-bar-height: ${$mobileStatusBarHeight}px; --mobile-navigation-bar-height: ${$mobileNavigationBarHeight}px;`}
>
    <div
        class={`md:row-[1] md:col-[1] p-6 md:p-0 flex items-end
        ${lyrics.length > 1 ? "justify-end" : "justify-center"}`}
    >
        <div
            class={`w-full md:w-[80%] xl:w-[65%] text-white ${lyrics.length > 0 && "ms-auto"}`}
        >
            <img
                class="w-full rounded-lg aspect-square"
                src={albumImage}
                alt="Music Album"
            />
        </div>
    </div>
    <div
        class={`md:row-[2] md:col-[1] order-last md:order-2 p-5 md:p-0 mb-10 md:pb-0 flex ${lyrics.length > 0 ? "justify-end" : "justify-center"}`}
    >
        <div class="w-full md:w-[80%] xl:w-[65%]">
            <div class="w-full grid grid-cols-[auto,1fr,auto] mt-4">
                <div class="text-xs lg:text-sm flex w-12">
                    <span class="self-end opacity-75"
                        >{progressDurationText}</span
                    >
                </div>
                <div class="text-lg font-medium text-center mt-2 opacity-90">
                    <p>
                        {music?.albumArtist ??
                            music?.artist ??
                            MusicConfig.defaultArtist} {MusicConfig.separator}
                        {music?.title ?? MusicConfig.defaultTitle}
                    </p>
                </div>
                <div class="text-xs lg:text-sm flex justify-end w-12">
                    <span class="self-end opacity-75"
                        >{progressDurationNegativeText}</span
                    >
                </div>
            </div>
            <div class="w-full pt-4 pb-2 relative">
                <input
                    class={`w-full absolute music-progress-bar-play`}
                    type="range"
                    style={`--progress-width: ${progressPercentage}%`}
                    bind:value={$musicProgressValue}
                    min={MusicConfig.min}
                    max={MusicConfig.max}
                    step={MusicConfig.step}
                    onchange={onPlayerBarChange}
                />
                <input
                    class={`w-full absolute music-progress-bar-play-end`}
                    type="range"
                    style={`--progress-width: ${progressPercentage}%`}
                    bind:value={$musicProgressValue}
                    min={MusicConfig.min}
                    max={MusicConfig.max}
                    step={MusicConfig.step}
                    onchange={onPlayerBarChange}
                />
            </div>
            <div class="w-full grid grid-cols-5 mt-4">
                <div class="grid items-center">
                    <button
                        class="w-8 xl:w-9 invert mx-2"
                        onclick={handleButtonBack}
                        ><img
                            src={MusicConfig.defaultBackButton}
                            alt="Icon Back"
                        /></button
                    >
                </div>
                <div class="flex justify-end">
                    <button
                        class="w-12 md:w-10 tb:w-10 lg:w-11 invert mx-2"
                        onclick={handleButtonPrevious}
                        ><img
                            src={MusicConfig.defaultPreviousButton}
                            alt="Icon Previous"
                        /></button
                    >
                </div>
                <div class="flex justify-center">
                    <button
                        class="w-12 md:w-10 tb:w-10 lg:w-11 invert mx-2"
                        onclick={handleButtonPlayPause}
                        ><img
                            src={$musicIsPlaying
                                ? MusicConfig.defaultPauseButton
                                : MusicConfig.defaultPlayButton}
                            alt="Icon Play"
                        /></button
                    >
                </div>
                <div class="flex justify-start">
                    <button
                        class="w-12 md:w-10 tb:w-10 lg:w-11 invert mx-2"
                        onclick={handleButtonNext}
                        ><img
                            src={MusicConfig.defaultNextButton}
                            alt="Icon Next"
                        /></button
                    >
                </div>
            </div>
            <div class="mt-5 volume-action animate__animated animate__faster">
                <div class="grid grid-cols-[auto_1fr_auto] items-center gap-3">
                    <button onclick={() => MusicController.setVolume(0)}>
                        <img
                            class="invert w-5"
                            alt="Volume"
                            src={MusicConfig.defaultMuteButton}
                        />
                    </button>
                    <div class="relative">
                        <input
                            class={`absolute w-full volume-progress-bar-end`}
                            type="range"
                            style={`--progress-width: ${volumePercentage}%`}
                            min={MusicConfig.vmin}
                            max={MusicConfig.vmax}
                            step={MusicConfig.vstep}
                        />
                        <input
                            class={`absolute w-full volume-progress-bar`}
                            type="range"
                            style={`--progress-width: ${volumePercentage}%`}
                            bind:value={$musicVolume}
                            min={MusicConfig.vmin}
                            max={MusicConfig.vmax}
                            step={MusicConfig.vstep}
                        />
                    </div>
                    <button onclick={() => MusicController.setVolume(1)}>
                        <img
                            class="invert w-5"
                            alt="Volume"
                            src={MusicConfig.defaultSpeakerButton}
                        />
                    </button>
                </div>
            </div>
        </div>
    </div>
    {#if lyrics.length > 0}
        <div
            class="w-full md:h-screen md:row-[1/span_2] md:col-[2] px-6 md:px-20 overflow-y-auto overflow-x-hidden scrollbar-hidden
            [mask-image:linear-gradient(to_bottom,rgba(0,0,0,1)_60%,rgba(0,0,0,0))]
            md:[mask-image:linear-gradient(to_bottom,rgba(0,0,0,0),rgba(0,0,0,1),rgba(0,0,0,0))]
            animate__animated animate__faster animate__fadeInUp"
        >
            <div class="flex">
                <div
                    id="lyrics"
                    class="w-full md:w-[55vw] h-full md:my-[40vh] font-bold text-[1.5rem] xl:text-[2rem]"
                >
                    {#each lyrics as lyric, i}
                        {#if selectedLyricIndex == i}
                            <p
                                id="selected-lyric"
                                class="text-[1.65rem] xl:text-[2.15rem] py-5 md:py-7 lg:py-10"
                            >
                                {lyric.lyric}
                            </p>
                        {:else}
                            <p class="opacity-50 py-5 md:py-7 lg:py-10">
                                {lyric.lyric}
                            </p>
                        {/if}
                    {/each}
                </div>
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    @media (min-width: 40rem) {
        .volume-action {
            animation-name: fadeOut;
            &:hover {
                animation-name: fadeIn;
            }
        }
    }

    #root-play {
        --margin-vertical: calc(
            (var(--mobile-status-bar-height) / 2) +
                (var(--mobile-navigation-bar-height) / 2)
        );
        margin-top: var(--margin-vertical);
        margin-bottom: var(--margin-vertical);
        grid-template-rows:
            auto calc(
                (30% - var(--mobile-status-bar-height)) - var(
                        --mobile-navigation-bar-height
                    )
            )
            auto;

        @media (min-width: 40rem) {
            margin-top: 0;
            margin-bottom: 0;
            grid-template-rows: auto 45% auto;
        }

        @media (min-width: 48rem) {
            grid-template-rows: auto 40% auto;
        }

        @media (min-width: 64rem) {
            grid-template-rows: auto 35% auto;
        }
    }
    .root-nolyrics {
        @media (min-width: 40rem) {
            grid-template-rows: 55% 45% !important;
        }

        @media (min-width: 48rem) {
            grid-template-rows: 60% 40% !important;
        }

        @media (min-width: 64rem) {
            grid-template-rows: 65% 35% !important;
        }
    }

    /* @keyframes grow-grid {
      from {
        grid-template-columns: 40% 0%;
      }
      to {
        grid-template-columns: 40% 55%;
      }
    }

    .animate-grow-grid {
      animation: grow-grid 1s ease-in-out;
    } */
</style>
