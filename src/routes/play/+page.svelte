<script lang="ts">
import {
	musicCurrentIndex,
	musicIsPlaying,
	musicProgressValue,
	musicRepeatMode,
	musicVolume,
} from "$lib/stores/music";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import type MusicLyric from "$lib/home/music/lyric";
import LrcLib from "$lib/api/lrclib";
import {
	mobileNavigationBarHeight,
	mobileStatusBarHeight,
} from "$lib/stores/mobile";
import { isAndroid, isMacos, isMobile } from "$lib/platform";
import PageController from "$lib/controllers/PageController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { RepeatMode } from "$lib/home/music/types";
import {settingUiPlayShowBackButton, settingUiShowRepeatButton, settingUiShowShuffleButton} from "$lib/stores/setting";

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

const unlistenMusicProgressValue = musicProgressValue.subscribe(() => {
	progressPercentage = MusicController.progressPercentage();
	progressDurationText = MusicController.progressDurationText();
	progressDurationNegativeText = MusicController.progressDurationText(true);

	resetSelectedLyricIndex();
});
const unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(() => {
	music = MusicController.currentMusic();
	albumImage = MusicController.currentMusicAlbumImage();
	resetLyrics();
});

const unlistenMusicVolume = musicVolume.subscribe(() => {
	volumePercentage = MusicController.volumePercentage();
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
		MusicController.pause();
	} else MusicController.play();
}

function handleButtonPrevious() {
	MusicController.previousMusic();
}

function handleButtonNext() {
	MusicController.nextMusic();
}

function handleButtonBack() {
	unlistenMusicProgressValue();
	unlistenMusicCurrentIndex();
	unlistenMusicVolume();
	PageController.back();
}

async function onKeyDown(
	e: KeyboardEvent & {
		currentTarget: EventTarget & Document;
	},
) {
    if (e.key === "Escape") handleButtonBack();
	if (e.key === " ") handleButtonPlayPause();
}

async function resetLyrics() {
	selectedLyricIndex = 0;

	if (MusicController.currentMusic() == null) return;
	const resLyrics = await LrcLib.getLyrics(MusicController.currentMusic()!);
	if (resLyrics == null) {
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
</script>

<svelte:document onkeydown={onKeyDown} />

<div
    id="root-play"
    class={`w-full h-full grid mx-auto max-w-[35rem] md:max-w-none md:gap-y-0 md:pt-0
    ${lyrics.length > 1 ? "md:grid-cols-[40%_55%]" : "md:grid-cols-[45%] justify-center root-nolyrics"}
    ${isMacos() && "pt-6"}`}
    style={`--mobile-status-bar-height: ${$mobileStatusBarHeight}px; --mobile-navigation-bar-height: ${$mobileNavigationBarHeight}px;`}
>
    <div
        class={`md:row-[1] md:col-[1] ${isMobile() ? "p-5" : "p-4"} md:p-0 flex items-end
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
        class={`md:row-[2] md:col-[1] order-last md:order-2 ${isMobile() ? "px-5" : "px-4"} pb-5 pt-2 ${isMobile() && "mb-5"} md:p-0 md:pb-0 flex ${lyrics.length > 0 ? "justify-end" : "justify-center"}`}
    >
        <div class="w-full md:w-[80%] xl:w-[65%]">
            <div class="w-full grid grid-cols-[auto,1fr,auto] md:mt-4">
                <div class="text-xs lg-mdpi:text-sm flex w-12">
                    <span class="self-end opacity-75"
                        >{progressDurationText}</span
                    >
                </div>
                <div class="font-medium text-center mt-2 opacity-90 overflow-hidden
                    text-sm md-mdpi:text-base md-hdpi:text-sm lg-mdpi:text-base">
                    <!-- Note: Idk why the title scroll doesn't work without sacrificing first element -->
                    <p class="animate-scroll-overflow-text"></p>
                    <p class="whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text">
                        {music?.albumArtist ??
                            music?.artist ??
                            MusicConfig.defaultArtist} {MusicConfig.separator}
                        {music?.title ?? MusicConfig.defaultTitle}
                    </p>
                </div>
                <div class="text-xs lg-mdpi:text-sm flex justify-end w-12">
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
                    oninput={MusicController.onPlayerBarChange}
                />
                <input
                    class={`w-full absolute music-progress-bar-play-end`}
                    type="range"
                    style={`--progress-width: ${progressPercentage}%`}
                    bind:value={$musicProgressValue}
                    min={MusicConfig.min}
                    max={MusicConfig.max}
                    step={MusicConfig.step}
                    oninput={MusicController.onPlayerBarChange}
                />
            </div>
            <div class={`w-full grid ${isAndroid() || !$settingUiPlayShowBackButton ? "grid-cols-[1fr_auto_auto_auto_1fr]" : "grid-cols-7"} items-center gap-2 mt-4`}>
                {#if !isAndroid() && $settingUiPlayShowBackButton}
                    <div class="flex items-center">
                        <button
                            id="btn-back"
                            class="w-7 md-mdpi:w-8 md-hdpi:w-8 mx-2 animate__animated"
                            onclick={handleButtonBack}
                        ><Icon type={IconType.PlayBack} /></button>
                    </div>
                {/if}
                <div class="flex justify-end">
                    {#if $settingUiShowRepeatButton}
                        <button class={`w-7 md-mdpi:w-8 md-hdpi:w-8 mx-2 ${$musicRepeatMode === RepeatMode.None ? "opacity-80" : ""}`}
                                onclick={MusicController.toggleRepeatMode}>
                            {#if $musicRepeatMode === RepeatMode.All}
                                <Icon type={IconType.Repeat} />
                            {:else if $musicRepeatMode === RepeatMode.None}
                                <Icon type={IconType.RepeatPlayNone} />
                            {:else if $musicRepeatMode === RepeatMode.One}
                                <Icon type={IconType.RepeatOne} />
                            {/if}
                        </button>
                    {/if}
                </div>
                <div class="flex justify-end">
                    <button
                        class="w-12 sm:w-14 md-mdpi:w-12 md-hdpi:w-12 lg-mdpi:w-14"
                        onclick={handleButtonPrevious}
                        ><Icon type={IconType.Previous} /></button
                    >
                </div>
                <div class="flex justify-center">
                    <button
                        class="w-12 sm:w-14 md-mdpi:w-12 md-hdpi:w-12 lg-mdpi:w-14"
                        onclick={handleButtonPlayPause}
                        >
                        {#if $musicIsPlaying}
                            <Icon type={IconType.Pause} />
                        {:else}
                            <Icon type={IconType.Play} />
                        {/if}
                    </button>
                </div>
                <div class="flex justify-start">
                    <button
                        class="w-12 sm:w-14 md-mdpi:w-12 md-hdpi:w-12 lg-mdpi:w-14"
                        onclick={handleButtonNext}
                        ><Icon type={IconType.Next} /></button
                    >
                </div>
                <div class="flex justify-start">
                    {#if $settingUiShowShuffleButton}
                        <button class="w-7 md-mdpi:w-8 md-hdpi:w-8 mx-2" onclick={() => MusicController.playShuffle()}>
                            <Icon type={IconType.Shuffle} />
                        </button>
                    {/if}
                </div>
            </div>
            <div id="volume-bar" class="mt-5 animate__animated">
                <div class="grid grid-cols-[auto_1fr_auto] items-center gap-3">
                    <button class="w-5" onclick={() => MusicController.setVolume(0)}>
                        <Icon type={IconType.Mute} />
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
                    <button class="w-5" onclick={() => MusicController.setVolume(1)}>
                        <Icon type={IconType.Speaker} />
                    </button>
                </div>
            </div>
        </div>
    </div>
    {#if lyrics.length > 0}
        <div
            class={`w-full md:h-screen md:row-[1/span_2] md:col-[2] md:px-20 overflow-y-auto overflow-x-hidden
            scrollbar-hidden [mask-image:linear-gradient(to_bottom,rgba(0,0,0,1)_60%,rgba(0,0,0,0))]
            md:[mask-image:linear-gradient(to_bottom,rgba(0,0,0,0),rgba(0,0,0,1),rgba(0,0,0,0))]
            animate__animated animate__faster animate__fadeInUp ${isMobile() ? "px-5" : "px-4"}`}
        >
            <div class="flex">
                <div
                    id="lyrics"
                    class="w-full md:w-[55vw] h-full md:my-[40vh] font-bold
                    text-[1.15rem] md:text-[1.4rem] lg:text-[1.7rem]
                    md-hdpi:text-[1.15rem] lg-hdpi:text-[1.4rem]"
                >
                    {#each lyrics as lyric, i}
                        <p
                            id={selectedLyricIndex === i ? "selected-lyric" : ""}
                            class={selectedLyricIndex === i ? "text-[1.25rem] md:text-[1.55rem] lg:text-[1.85rem]" +
                             "md-hdpi:text-[1.25rem] lg-hdpi:text-[1.55rem]" +
                             "py-5 md:py-7 lg:py-10"
                                : "opacity-50 py-5 md:py-7 lg:py-10"}
                        >
                            {#if lyric.value.length > 0}
                                {lyric.value}
                            {:else}
                                <div class={`${selectedLyricIndex === i ? 'w-[1.35rem] md:w-[1.85rem] lg:w-[2.25rem]'
                                : 'w-[1.2rem] md:w-[1.7rem] lg:w-[2.1rem]'}`}>
                                    <Icon type={IconType.Note} />
                                </div>
                            {/if}
                        </p>
                    {/each}
                </div>
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    @media (min-width: 40rem) {
        #btn-back,
        #volume-bar {
            opacity: 0;
            transition: opacity 1s ease 3s;
            &:hover {
                opacity: 1;
                transition-delay: 0s;
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
        grid-template-rows: auto calc((auto - var(--mobile-status-bar-height)) - var(--mobile-navigation-bar-height)) auto;

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
