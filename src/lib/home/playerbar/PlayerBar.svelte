<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import {
	musicCurrentIndex,
	musicIsPlaying,
	musicPlaylist,
	musicProgressValue,
	musicRepeatMode,
	musicVolume,
} from "$lib/stores/music";
import { mobileNavigationBarHeight } from "$lib/stores/mobile";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { onDestroy, onMount } from "svelte";
import {type MusicData, MusicSize, RepeatMode} from "$lib/home/music/types";
import {
	settingUiShowRepeatButton,
	settingUiShowShuffleButton,
} from "$lib/stores/setting";
import type { Unsubscriber } from "svelte/store";
import Glass from "$lib/glass/Glass.svelte";
import {playerBarHeight} from "$lib/stores/playerbar";

const tooltipMargin = 12;

let element: HTMLDivElement;
let oldMusic: MusicData | null = $state(null);
let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $derived(MusicController.getAlbumImageFromMusic(oldMusic));

let isPlaying = $derived($musicIsPlaying);
let progressPercentage = $state(MusicController.progressPercentage());
let volumePercentage = $state(MusicController.volumePercentage());

let tooltip: HTMLDivElement;
let tooltipPosition = $state(0);
let tooltipVisible = $state(false);
let tooltipText = $state("0:00");

let touchLastX = $state(0);

const gridRight = (() => {
	if ($settingUiShowRepeatButton && $settingUiShowShuffleButton)
		return "grid-cols-[repeat(5,auto)]";
	if ($settingUiShowRepeatButton || $settingUiShowShuffleButton)
		return "grid-cols-[repeat(4,auto)]";
	return "grid-cols-[repeat(3,auto)]";
})();

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

function refresh() {
	setTimeout(async () => {
		let music = MusicController.currentMusic();

		if (music === null) {
			title = MusicConfig.defaultTitle;
			artist = MusicConfig.defaultArtist;
			return;
		}

		if (oldMusic !== null && oldMusic.path === music.path) return;

		oldMusic = music;
		title = music.title!;
		artist = music.artist;
	}, 0);
}

function updateTooltip(
	e: MouseEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	const rect = e.currentTarget.getBoundingClientRect();
	const x = e.clientX - rect.left;
	tooltipPosition = x - tooltip.offsetWidth / 2;
	if (tooltipPosition < tooltipMargin) tooltipPosition = tooltipMargin;
	else if (tooltipPosition + tooltip.offsetWidth > window.innerWidth - tooltipMargin)
		tooltipPosition = window.innerWidth - tooltip.offsetWidth - tooltipMargin;

	const percentage = (x / (window.innerWidth - tooltipMargin * 2)) * 100;
	tooltipText =
		MusicController.parsePercentageProgressDurationIntoText(percentage);
	tooltipVisible = true;
}

function updateTooltipTouch(
	e: TouchEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
    const rect = e.currentTarget.getBoundingClientRect();
    const touch = e.touches[0];
    const x = touch.clientX - rect.left;
    tooltipPosition = x - tooltip.offsetWidth / 2;
    if (tooltipPosition < tooltipMargin) tooltipPosition = tooltipMargin;
    else if (tooltipPosition + tooltip.offsetWidth > window.innerWidth - tooltipMargin)
        tooltipPosition = window.innerWidth - tooltip.offsetWidth - tooltipMargin;

    const percentage = (x / (window.innerWidth - tooltipMargin * 2)) * 100;
    tooltipText =
        MusicController.parsePercentageProgressDurationIntoText(percentage);
    tooltipVisible = true;

	touchLastX = x;
}
function hideTooltip() {
	tooltipVisible = false;
}

function updateProgress(
	e: MouseEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	const rect = e.currentTarget.getBoundingClientRect();
	const x = e.clientX - rect.left;
	const percentage = (x / (window.innerWidth - (tooltipMargin * 2))) * 100;
	MusicController.updateProgressByPercentage(percentage);
}

function updateProgressTouch(
	e: TouchEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	const percentage = (touchLastX / window.innerWidth) * 100;
	MusicController.updateProgressByPercentage(percentage);

	hideTooltip();
}

function updatePlayerBarHeight(){
    $playerBarHeight = element.offsetHeight;
}

let unlistenMusicProgressValue: Unsubscriber;
let unlistenMusicVolume: Unsubscriber;
let unlistenMusicCurrentIndex: Unsubscriber;
let unlistenMusicPlaylist: Unsubscriber;

onMount(() => {
	unlistenMusicProgressValue = musicProgressValue.subscribe(
		() => (progressPercentage = MusicController.progressPercentage()),
	);
	unlistenMusicVolume = musicVolume.subscribe(
		() => (volumePercentage = MusicController.volumePercentage()),
	);
	unlistenMusicCurrentIndex = musicCurrentIndex.subscribe(refresh);
	unlistenMusicPlaylist = musicPlaylist.subscribe(refresh);
    updatePlayerBarHeight();
});

onDestroy(() => {
	unlistenMusicProgressValue();
	unlistenMusicVolume();
	unlistenMusicCurrentIndex();
	unlistenMusicPlaylist();
});
</script>

<svelte:window onresize={updatePlayerBarHeight} />
<svelte:document onkeydown={onKeyDown} />

<div class="absolute bottom-0 w-full px-3 pt-3 animate__animated animate__slideInUp"
    style="padding-bottom: {$mobileNavigationBarHeight > 12 ? $mobileNavigationBarHeight : 12}px;"
    bind:this={element}>
    <div class="w-fit absolute top-[-1.5rem] border rounded-lg px-2 py-1 shadow-xl text-sm backdrop-blur-xl"
    style="
        left: {tooltipPosition}px;
        visibility: {tooltipVisible ? 'visible' : 'hidden'};
    "
    bind:this={tooltip}>{tooltipText}</div>
    <div class="relative mb-3">
        <div class="absolute w-full h-10 bottom-[-20px] left-0 cursor-pointer z-10"
             onmouseenter={updateTooltip}
             onmousemove={updateTooltip}
             onmouseleave={hideTooltip}
             ontouchstart={updateTooltipTouch}
             ontouchmove={updateTooltipTouch}
             ontouchend={updateProgressTouch}
             onclick={updateProgress}></div>
        <input
                class="w-full absolute backdrop-blur-lg music-progress-bar"
                type="range"
                style="--progress-width: {progressPercentage}%"
                bind:value={$musicProgressValue}
                min={MusicConfig.min}
                max={MusicConfig.max}
                step={MusicConfig.step}
        />
        <input
                class="w-full absolute music-progress-bar-end"
                type="range"
                style="--progress-width: {progressPercentage}%"
                bind:value={$musicProgressValue}
                min={MusicConfig.min}
                max={MusicConfig.max}
                step={MusicConfig.step}
        />
    </div>
    <Glass class="bg-gray-300/20"
        padding="0.5rem"
        paddingHover="0.7rem"
        enableBlur={true}
        enableHoverAnimation={true}
        glassEffectScale={50}>
        <div class="w-full grid grid-cols-[auto_min-content] md:grid-cols-3 py-1">
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
                    class="ms-2 md:ms-0 md:flex items-center justify-center
                order-first md:order-none
                text-sm md:text-[15px]"
            >
                <div
                        class="grid grid-cols-[2.5rem_auto] md:grid-cols-[3rem_auto]"
                >
                    <button onclick={redirectToPlay}>
                        {#await albumImage}
                            <div class="w-full aspect-square"></div>
                        {:then image}
                            <img
                                    class="w-full rounded animate__animated animate__fadeIn"
                                    src={image}
                                    alt="Album"
                            />
                        {/await}
                    </button>
                    <div class="ms-3 overflow-hidden grid grid-rows-[auto_1fr_1fr]">
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
            <div class="hidden md:grid justify-end">
                <div class={`grid items-center gap-3 ${gridRight}`}>
                    {#if $settingUiShowRepeatButton}
                        <button class={`w-6 ${$musicRepeatMode === RepeatMode.None ? 'opacity-60' : ''}`}
                                onclick={MusicController.toggleRepeatMode}>
                            {#if $musicRepeatMode === RepeatMode.All}
                                <Icon type={IconType.Repeat} />
                            {:else if $musicRepeatMode === RepeatMode.None}
                                <Icon type={IconType.RepeatNone} />
                            {:else if $musicRepeatMode === RepeatMode.One}
                                <Icon type={IconType.RepeatOne} />
                            {/if}
                        </button>
                    {/if}
                    {#if $settingUiShowShuffleButton}
                        <button class="w-6" onclick={() => MusicController.playShuffle()}>
                            <Icon type={IconType.Shuffle} />
                        </button>
                    {/if}
                    <button class="w-6" onclick={handleVolumeButton}>
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
    </Glass>
</div>