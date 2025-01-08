<script lang="ts">
    import {
        musicCurrent,
        musicIsPlaying,
        musicProgressValue,
    } from "$lib/stores/music";
    import "./page.scss";
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
    // Note: Used for testing
    // MusicController.getMusics();
</script>

<div class="w-full h-full grid items-center justify-center">
    <div class="w-[36rem]">
        <div class="w-full text-white">
            <div class="w-full grid items-center justify-center">
                <img class="rounded-lg" src={albumImage} alt="Music Album" />
            </div>
            <div class="w-full grid grid-cols-[auto,1fr,auto] mt-4">
                <div class="text-sm flex">
                    <span class="self-end">{progressDurationText}</span>
                </div>
                <div class="font-medium text text-center mt-2">
                    {music?.albumArtist ?? MusicConfig.defaultArtist} - {music?.title ??
                        MusicConfig.defaultTitle}
                </div>
                <div class="text-sm flex justify-end">
                    <span class="self-end">{progressDurationNegativeText}</span>
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
            <div class="w-full grid grid-cols-3 mt-2">
                <div class="flex justify-end">
                    <!-- TODO: Button Previous Functionality -->
                    <button class="w-8 md:w-10 tb:w-10 lg:w-12 invert mx-2"
                        ><img
                            src={MusicConfig.defaultPreviousButton}
                            alt="Icon Previous"
                        /></button
                    >
                </div>
                <div class="flex justify-center">
                    <button
                        class="w-8 md:w-10 tb:w-10 lg:w-12 invert mx-2"
                        onclick={handleButtonPlayPause}
                        ><img
                            src={$musicIsPlaying
                                ? MusicConfig.defaultPauseButton
                                : MusicConfig.defaultPlayButton}
                            alt="Icon Play"
                        /></button
                    >
                </div>
                <div>
                    <button
                        class="w-8 md:w-10 tb:w-10 lg:w-12 invert mx-2"
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
</div>
