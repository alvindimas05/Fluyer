<script lang="ts">
    import MusicController, {
        MusicConfig,
    } from "$lib/controllers/MusicController";
    import "./playerbar.scss";
    import {
        musicCurrent,
        musicIsPlaying,
        musicProgressValue,
    } from "$lib/stores/music";
    import { invoke } from "@tauri-apps/api/core";
    import { isMobile } from "$lib/platform";

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
        if (MusicController.isCurrentMusicFinished()) return;

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
        if (MusicController.isCurrentMusicFinished()) {
            MusicController.setProgressValue(0);
            return;
        }

        if (MusicController.progressValue() >= MusicConfig.max) {
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
        if(e.key === ' ') handleButtonPlayPause();
    }

    async function getNavigationBarHeight() {
        const navbarHeight = await invoke<number>("get_navigation_bar_height");
        navigationBarHeight = navbarHeight > 32 ? 32 : navbarHeight;
    }
    if (isMobile()) getNavigationBarHeight();
</script>

<svelte:document onkeydown={onKeyDown} />

<div
    class="fixed left-0 bottom-0 z-10 w-full bg-gray-700 bg-opacity-30 backdrop-blur-md text-white animate__animated animate__slideInUp animate__slow"
    style={`padding-bottom: ${navigationBarHeight}px`}
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
                <button class="w-8 lg:w-10 invert mx-2"
                    ><img
                        class="music-icon"
                        src="/icons/default/previous.png"
                        alt="Icon Previous"
                    /></button
                >
                <button
                    class="w-8 lg:w-10 invert mx-2"
                    onclick={handleButtonPlayPause}
                    ><img
                        class="music-icon"
                        src={isPlaying
                            ? MusicConfig.defaultPauseButton
                            : MusicConfig.defaultPlayButton}
                        alt="Icon Play"
                    /></button
                >
                <button
                    class="w-8 lg:w-10 invert mx-2"
                    onclick={handleButtonNext}
                    ><img
                        class="music-icon"
                        src="/icons/default/next.png"
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
                    <img class="rounded" src={albumImage} alt="Album" />
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
