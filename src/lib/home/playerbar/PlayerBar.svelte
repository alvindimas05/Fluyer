<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import {
    musicCurrentIndex,
    musicIsPlaying,
    musicPlaylist,
    musicProgressValue,
    musicRepeatMode,
    musicVolume,
} from "$lib/stores/music.svelte";
import { mobileNavigationBarHeight } from "$lib/stores/mobile.svelte";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { onDestroy, onMount } from "svelte";
import { type MusicData, MusicSize, RepeatMode } from "$lib/home/music/types";
import {
    settingBitPerfectMode,
    settingUiShowRepeatButton,
    settingUiShowShuffleButton,
} from "$lib/stores/setting";
import type { Unsubscriber } from "svelte/store";
import Glass from "$lib/glass/Glass.svelte";
import { playerBarHeight } from "$lib/stores/playerbar.svelte";
import View from "$lib/ui/components/View.svelte";
import ProgressBar from "$lib/ui/components/ProgressBar.svelte";

let element: HTMLDivElement;
let oldMusic: MusicData | null = $state(null);
let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $derived(MusicController.getAlbumImageFromMusic(oldMusic));

let isPlaying = $derived($musicIsPlaying);
let progressPercentage = $state(MusicController.progressPercentage);
let volumePercentage = $state(MusicController.volumePercentage);

const gridRight = (() => {
    if ($settingUiShowRepeatButton && $settingUiShowShuffleButton)
        return "grid-cols-[repeat(5,auto)]";
    if ($settingUiShowRepeatButton || $settingUiShowShuffleButton)
        return "grid-cols-[repeat(4,auto)]";
    return "grid-cols-[repeat(3,auto)]";
})();

function handleButtonPlayPause() {
    if (MusicController.isPlaying) {
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

function redirectToPlay() {
    PageController.goto(PageRoutes.PLAY);
}

function handleVolumeButton() {
    MusicController.setVolume(MusicController.volume > 0 ? 0 : 1);
}

function refresh() {
    setTimeout(async () => {
        let music = MusicController.currentMusic;

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

function handleProgressClick(percentage: number) {
    MusicController.seekByPercentage(percentage);
}

function handleVolumeProgressClick(percentage: number) {
    MusicController.setVolume(percentage / 100);
}

function updatePlayerBarHeight() {
    $playerBarHeight = element.offsetHeight;
}

let unlistenMusicProgressValue: Unsubscriber;
let unlistenMusicVolume: Unsubscriber;
let unlistenMusicCurrentIndex: Unsubscriber;
let unlistenMusicPlaylist: Unsubscriber;

onMount(() => {
    unlistenMusicProgressValue = musicProgressValue.subscribe(
        () => (progressPercentage = MusicController.progressPercentage),
    );
    unlistenMusicVolume = musicVolume.subscribe(
        () => (volumePercentage = MusicController.volumePercentage),
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

<div class="absolute bottom-0 w-full px-3 pt-3 animate__animated animate__slideInUp"
     style="padding-bottom: {$mobileNavigationBarHeight > 12 ? $mobileNavigationBarHeight : 12}px;"
     bind:this={element}>

    <ProgressBar
            bind:value={$musicProgressValue}
            {progressPercentage}
            min={MusicConfig.min}
            max={MusicConfig.max}
            step={MusicConfig.step}
            showTooltip={true}
            tooltipFormatter={(percentage) => MusicController.parsePercentageProgressDurationIntoText(percentage)}
            onProgressClick={handleProgressClick}
            class="mb-3"
            size="lg"
    />

    <View class="bg-gray-400/35 px-3 py-2 hover:px-4 hover:py-3 rounded-full"
          glassEnableBlur={true}>
        <div class="w-full grid grid-cols-[auto_min-content] md:grid-cols-3 py-1">
            <div class="flex items-center ps-1 sm:gap-x-1">
                <button
                        class="hidden sm:block w-10 md:w-12 lg:w-12"
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
                        class="hidden sm:block w-10 md:w-12 lg:w-12"
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
                                    class="w-full aspect-square object-cover rounded animate__animated animate__fadeIn"
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
                    <button class="w-6 {$settingBitPerfectMode ? 'pointer-events-none' : ''}" onclick={handleVolumeButton}>
                        {#if volumePercentage > 0}
                            <Icon type={IconType.Speaker} />
                        {:else}
                            <Icon type={IconType.Mute} />
                        {/if}
                    </button>
                    <div class="relative w-24 {$settingBitPerfectMode ? 'pointer-events-none' : ''}">
                        <ProgressBar
                                bind:value={$musicVolume}
                                progressPercentage={volumePercentage}
                                onProgressClick={handleVolumeProgressClick}
                                min={MusicConfig.vmin}
                                max={MusicConfig.vmax}
                                step={MusicConfig.vstep}
                                showTooltip={false}
                                class="w-24"
                                size="sm"
                        />
                    </div>
                </div>
            </div>
        </div>
    </View>
</div>