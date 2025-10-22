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
import {
    mobileNavigationBarHeight,
    mobileStatusBarHeight,
} from "$lib/stores/mobile";
import { isAndroid, isMacos, isMobile } from "$lib/platform";
import PageController from "$lib/controllers/PageController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { RepeatMode } from "$lib/home/music/types";
import {
    settingBitPerfectMode,
    settingUiPlayShowBackButton, settingUiPlayShowVolume,
    settingUiShowRepeatButton,
    settingUiShowShuffleButton,
} from "$lib/stores/setting";
import { showThenFade } from "$lib/controllers/UIController";
import Glass from "$lib/glass/Glass.svelte";
import LyricController from "$lib/controllers/LyricController";
import View from "$lib/components/View.svelte";

let music = $state(MusicController.currentMusic());
let progressPercentage = $state(MusicController.progressPercentage());
let progressDurationText = $state(MusicController.progressDurationText());
let progressDurationNegativeText = $state(
    MusicController.progressDurationText(true),
);
let albumImage = $state(MusicController.currentMusicAlbumImage());

let lyrics: MusicLyric[] = $state([]);
let selectedLyricIndex = $state(0);
let volumePercentage = $state(MusicController.volumePercentage());

let progressBar: HTMLDivElement;
let updateProgressText = $state(true);
let touchLastX = $state(0);

const unlistenMusicProgressValue = musicProgressValue.subscribe(() => {
    progressPercentage = MusicController.progressPercentage();
    if (updateProgressText) {
        progressDurationText = MusicController.progressDurationText();
        progressDurationNegativeText = MusicController.progressDurationText(true);
    }

    resetSelectedLyricIndex();
});
const unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(async () => {
    music = MusicController.currentMusic();
    albumImage = MusicController.currentMusicAlbumImage();
    resetLyrics();
});

const unlistenMusicVolume = musicVolume.subscribe(() => {
    volumePercentage = MusicController.volumePercentage();
});

function handleButtonPlayPause() {
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
}

async function resetLyrics() {
    selectedLyricIndex = 0;

    if (MusicController.currentMusic() == null) return;
    const resLyrics = await LyricController.get(MusicController.currentMusic()!);
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

function setProgressText(
    e: MouseEvent & {
        currentTarget: EventTarget & HTMLDivElement;
    },
) {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percentage = (x / progressBar.offsetWidth) * 100;
    updateProgressText = false;
    progressDurationText =
        MusicController.parsePercentageProgressDurationIntoText(percentage);
    progressDurationNegativeText =
        MusicController.parsePercentageProgressDurationIntoText(percentage, true);
}

function setProgressTextTouch(
    e: TouchEvent & {
        currentTarget: EventTarget & HTMLDivElement;
    },
) {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.touches[0].clientX - rect.left;
    const percentage = (x / progressBar.offsetWidth) * 100;
    updateProgressText = false;
    progressDurationText =
        MusicController.parsePercentageProgressDurationIntoText(percentage);
    progressDurationNegativeText =
        MusicController.parsePercentageProgressDurationIntoText(percentage, true);

    touchLastX = x;
}

function updateProgress(
    e: MouseEvent & {
        currentTarget: EventTarget & HTMLDivElement;
    },
) {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percentage = (x / progressBar.offsetWidth) * 100;
    MusicController.updateProgressByPercentage(percentage);

    resetProgressText();
}

function updateProgressTouch(
    e: TouchEvent & {
        currentTarget: EventTarget & HTMLDivElement;
    },
) {
    const percentage = (touchLastX / progressBar.offsetWidth) * 100;
    MusicController.updateProgressByPercentage(percentage);

    resetProgressText();
}

function resetProgressText() {
    updateProgressText = true;
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
    ${lyrics.length > 1 ? "md:grid-cols-[40%_55%]" : "md:grid-cols-[50%] justify-center root-nolyrics"}
    ${isMacos() && "pt-6"}`}
    style={`--mobile-status-bar-height: ${$mobileStatusBarHeight}px; --mobile-navigation-bar-height: ${$mobileNavigationBarHeight}px;`}
>
    <div
        class={`md:row-[1] md:col-[1] ${isMobile() ? "p-5" : "p-4"} md:p-0 flex items-end
        ${lyrics.length > 1 ? "justify-end" : "justify-center"}`}
    >
        <div
            class="w-full {lyrics.length > 0 && 'ms-auto'}
            md-mdpi:w-[80%] lg-mdpi:w-[75%] xl-mdpi:w-[65%]
            md-hdpi:w-[90%] lg-hdpi:w-[80%] xl-hdpi:w-[70%]
            md-xhdpi:w-[80%] lg-xhdpi:w-[70%]"
        >
            {#await albumImage}
                <div class="w-full aspect-square"></div>
            {:then image}
                <img
                    class="w-full rounded-lg object-cover aspect-square shadow-lg animate__animated animate__fadeIn animate__faster"
                    src={image}
                    alt="Music Album"
                />
            {/await}
        </div>
    </div>
    <div
        class="md:row-[2] md:col-[1] order-last md:order-2 {isMobile() ? 'px-5' : 'px-4'} pb-5 pt-2 {isMobile() && 'mb-5'}
        md:p-0 md:pb-0 flex {lyrics.length > 0 ? 'justify-end' : 'justify-center'}"
    >
        <View class="w-full h-fit md:mt-4 rounded-xl
            px-4 py-5 hover:py-7 hover:px-5
            md-mdpi:w-[80%] lg-mdpi:w-[75%] xl-mdpi:w-[65%]
            md-hdpi:w-[90%] lg-hdpi:w-[80%] xl-hdpi:w-[70%]
            md-xhdpi:w-[80%] lg-xhdpi:w-[70%]">
            <div class="w-full grid grid-cols-[auto,1fr,auto]">
                <div class="text-xs lg-mdpi:text-sm flex w-12">
                    <span class="self-end opacity-75"
                        >{progressDurationText}</span
                    >
                </div>
                <div
                    class="font-medium text-center mt-2 opacity-90 overflow-hidden
                    text-sm md-mdpi:text-base md-hdpi:text-base md-xhdpi:text-sm lg-xhdpi:text-base"
                >
                    <!-- Note: Idk why the title scroll doesn't work without sacrificing first element -->
                    <p class="animate-scroll-overflow-text"></p>
                    <p
                        class="whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text"
                    >
                        {music?.albumArtist ??
                            music?.artist ??
                            MusicConfig.defaultArtist}
                        {MusicConfig.separator}
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
                />
                <input
                    class={`w-full absolute music-progress-bar-play-end`}
                    type="range"
                    style={`--progress-width: ${progressPercentage}%`}
                    bind:value={$musicProgressValue}
                    min={MusicConfig.min}
                    max={MusicConfig.max}
                    step={MusicConfig.step}
                />
                <div class="w-full h-5 absolute left-0 top-[8px] cursor-pointer"
                    bind:this={progressBar}
                    onmouseenter={setProgressText}
                    onmousemove={setProgressText}
                    onmouseleave={resetProgressText}
                    ontouchstart={setProgressTextTouch}
                    ontouchmove={setProgressTextTouch}
                    ontouchend={updateProgressTouch}
                    onclick={updateProgress}></div>
            </div>
            <div
                class={`w-full grid ${isAndroid() || !$settingUiPlayShowBackButton ? "grid-cols-[1fr_auto_auto_auto_1fr]" : "grid-cols-7"} items-center gap-2 mt-4`}
            >
                {#if !isAndroid() && $settingUiPlayShowBackButton}
                    <div class="flex items-center">
                        <button
                            id="btn-back"
                            class="w-7 md-mdpi:w-8 md-hdpi:w-8 mx-2 animate__animated show-then-fade"
                            use:showThenFade
                            onclick={handleButtonBack}
                            ><Icon type={IconType.PlayBack} /></button
                        >
                    </div>
                {/if}
                <div class="flex justify-end">
                    {#if $settingUiShowRepeatButton}
                        <button
                            class={`w-7 md-mdpi:w-8 md-hdpi:w-8 mx-2 ${$musicRepeatMode === RepeatMode.None ? "opacity-80" : ""}`}
                            onclick={MusicController.toggleRepeatMode}
                        >
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
                        <button
                            class="w-7 md-mdpi:w-8 md-hdpi:w-8 mx-2"
                            onclick={() => MusicController.playShuffle()}
                        >
                            <Icon type={IconType.Shuffle} />
                        </button>
                    {/if}
                </div>
            </div>
            {#if $settingUiPlayShowVolume && !$settingBitPerfectMode}
                <div id="volume-bar" class="mt-5">
                    <div class="grid grid-cols-[auto_1fr_auto] items-center gap-3">
                        <button
                                class="w-5"
                                onclick={() => MusicController.setVolume(0)}
                        >
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
                        <button
                                class="w-5"
                                onclick={() => MusicController.setVolume(1)}
                        >
                            <Icon type={IconType.Speaker} />
                        </button>
                    </div>
                </div>
            {/if}
        </View>
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
                    class="w-full md:w-[55vw] h-full md:my-[40vh] font-bold text-[1.15rem]
                    sm-mdpi:text-[1.25rem] md-mdpi:text-[1.4rem] lg-mdpi:text-[1.7rem]
                    md-hdpi:text-[1.3rem] lg-hdpi:text-[1.45rem]
                    md-xhdpi:text-[1.2rem] lg-xhdpi:text-[1.4rem]"
                >
                    {#each lyrics as lyric, i}
                        <div
                            id={selectedLyricIndex === i
                                ? "selected-lyric"
                                : ""}
                            class={selectedLyricIndex === i
                                ? `text-[1.25rem]
                                    sm-mdpi:text-[1.35rem] md-mdpi:text-[1.55rem] lg-mdpi:text-[1.85rem]
                                    md-hdpi:text-[1.4rem] lg-hdpi:text-[1.6rem]
                                    md-xhdpi:text-[1.3rem] lg-xhdpi:text-[1.55rem]
                                    py-5 md:py-7 lg:py-10`
                                : "opacity-50 py-5 md:py-7 lg:py-10"}
                        >
                            {#if lyric.value.length > 0}
                                {lyric.value}
                            {:else}
                                <div
                                    class={`${
                                        selectedLyricIndex === i
                                            ? "w-[1.4rem] md:w-[1.9rem] lg:w-[2.25rem]"
                                            : "w-[1.25rem] md:w-[1.75rem] lg:w-[2.15rem]"
                                    }`}
                                >
                                    <Icon type={IconType.Note} />
                                </div>
                            {/if}
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    {/if}
</div>
