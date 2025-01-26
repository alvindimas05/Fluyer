<script lang="ts">
    import {
        musicCurrent,
        musicIsPlaying,
        musicProgressValue,
    } from "$lib/stores/music";
    import MusicController, {
        MusicConfig,
    } from "$lib/controllers/MusicController";

    // Based on Rust Rodio fade effect (Please check player.rs)
    let pauseDelay = 400;
    let music = $state(MusicController.currentMusic());
    let progressPercentage = $state(MusicController.progressPercentage());
    let progressDurationText = $state(MusicController.progressDurationText());
    let progressDurationNegativeText = $state(
        MusicController.progressDurationText(true),
    );
    let albumImage = $state(MusicConfig.defaultAlbumImage);

    musicProgressValue.subscribe(() => {
        progressPercentage = MusicController.progressPercentage();
        progressDurationText = MusicController.progressDurationText();
        progressDurationNegativeText =
            MusicController.progressDurationText(true);
    });
    musicCurrent.subscribe(() => {
        music = MusicController.currentMusic();
        albumImage = MusicController.currentMusicAlbumImage();
    });

    function handleButtonPlayPause() {
        if (
            MusicController.currentMusic() === null ||
            MusicController.isProgressValueEnd()
        ) {
            // Note: Used for testing
            // MusicController.addMusic(MusicController.musicList()![0]);
            return;
        }

        if (MusicController.isPlaying()) {
            MusicController.setIsPlaying(false);
            setTimeout(MusicController.pause, pauseDelay);
        } else MusicController.play();
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
    
    function backToHome(){
        window.history.back();
    }
    
    async function onKeyDown(
    	e: KeyboardEvent & {
    		currentTarget: EventTarget & Document;
    	},
    ) {
        if (e.key === " ") handleButtonPlayPause();
    }
    
    // Note: Used for testing
    // MusicController.getMusics();
</script>

<svelte:document onkeydown={onKeyDown} />

<div class="w-full h-full flex items-center justify-center">
    <div class="h-full grid gap-y-0 md:grid-cols-[40%_55%] justify-center">
        <div class="md:row-[1] md:col-[1] h-fit p-6 md:p-0 self-end">
            <div class="w-full md:w-[60%] text-white aspect-square ms-auto">
                <!-- <img class="rounded-lg w-full [mask-image:linear-gradient(to_right,rgba(0,0,0,0),rgba(0,0,0,1),rgba(0,0,0,0))] md:[mask-image:none]" src={albumImage} alt="Music Album" /> -->
                <!-- <img class="rounded-lg w-full [mask-image:radial-gradient(rgba(0,0,0,1),rgba(0,0,0,0))] md:[mask-image:none]" src={albumImage} alt="Music Album" /> -->
                <img class="rounded-lg w-full" src={albumImage} alt="Music Album" />
            </div>
        </div>
        <div class="md:row-[2] md:col-[1] order-last md:order-2 p-5 md:p-0 h-fit">
            <div class="w-full md:w-[60%] ms-auto">
                <div class="grid grid-cols-[auto,1fr,auto] mt-4 ms-auto">
                    <div class="text-xs lg:text-sm flex w-10">
                        <span class="self-end opacity-75">{progressDurationText}</span>
                    </div>
                    <div class="font-medium text-lg xl:text-xl text-center mt-2 opacity-90">
                        {music?.albumArtist ?? MusicConfig.defaultArtist} - {music?.title ??
                            MusicConfig.defaultTitle}
                    </div>
                    <div class="text-xs lg:text-sm flex justify-end w-10">
                        <span class="self-end opacity-75">{progressDurationNegativeText}</span>
                    </div>
                </div>
                <div class="w-full mt-[-4px]">
                    <input
                        id="music-progress-bar"
                        class="w-full"
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
                        <button class="w-6 md:w-8 tb:w-8 xl:w-9 invert mx-2"
                            onclick={backToHome}
                            ><img
                                src={MusicConfig.defaultBackButton}
                                alt="Icon Back"
                            /></button
                        >
                    </div>
                    <!-- TODO: Button Previous Functionality -->
                    <div class="flex justify-end">
                        <button class="w-10 xl:w-11 invert mx-2"
                        ><img
                            src={MusicConfig.defaultPreviousButton}
                            alt="Icon Previous"
                        /></button
                    >
                    </div>
                    <div class="flex justify-center">
                        <button
                            class="w-10 xl:w-11 invert mx-2"
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
                            class="w-10 xl:w-11 invert mx-2"
                            onclick={handleButtonNext}
                            ><img
                                src={MusicConfig.defaultNextButton}
                                alt="Icon Next"
                            /></button
                        >
                    </div>
                </div>
            </div>
        </div>
        <div class="md:row-[1/span_2] md:col-[2] mx-6 md:mx-20 overflow-auto scrollbar-hidden [mask-image:linear-gradient(to_bottom,rgba(0,0,0,1),rgba(0,0,0,1),rgba(0,0,0,0))]">
            <div class="w-full h-fit md:my-[40vh] font-bold text-[1.5rem] xl:text-[2rem]">
                <p class="py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
                <p class="opacity-50 py-5 md:py-10">Lorem Ipsum Dolor Sit Amet</p>
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
            background: linear-gradient(to right, #fff var(--progress-width), #ccd0d6 var(--progress-width));
        }
        &::-webkit-slider-thumb {
            @apply mt-[-6px] invisible;
        }
    }
</style>