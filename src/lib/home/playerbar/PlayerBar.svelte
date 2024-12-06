<script lang="ts">
    import Music, { MusicConfig } from "$lib/Music";
    import { invoke } from "@tauri-apps/api/core";
    import "./playerbar.scss";
    import { musicPlayed, musicIsPlaying } from "$lib/stores/music";

    let music = new Music();

    let musicValue = music.value;
    let musicProgressDuration = music.progressDuration;
    let musicProgressPercentage = music.progressPercentage;

    let albumArtist = "";
    let albumImage = "";

    $: musicValue,
        (() => {
            music.value = musicValue;
            updateStates();
        })();
    $: $musicIsPlaying, handlePlayPause();

    function handlePlayPause() {
        music.duration = ($musicPlayed?.duration ?? 0) / 1000;
        if ($musicIsPlaying) music.play();
        else music.pause();

        updateProgress();
    }

    function handleButtonPlayPause() {
        $musicIsPlaying = !$musicIsPlaying;
        handlePlayPause();
    }

    function updateProgress() {
        music.stopProgress();
        if (music.isPlaying) {
            music.startProgress(() => {
                musicValue = music.value;
                updateStates();
            });
        }
    }

    function updateStates() {
        musicProgressPercentage = music.progressPercentage;
        musicProgressDuration = music.progressDuration;

        albumArtist =
            $musicPlayed?.album_artist &&
            !$musicPlayed?.artist.includes($musicPlayed.album_artist)
                ? ` • ${$musicPlayed?.album_artist}`
                : "";
        albumImage = `data:image/png;base64,${$musicPlayed?.image}`;
    }
</script>

<div class="fixed left-0 bottom-0 w-full bg-[rgba(0,0,0,.5)] text-white">
    <input
        id="music-progress-bar"
        class="w-full absolute"
        type="range"
        style={`--progress-width: ${musicProgressPercentage}%`}
        bind:value={musicValue}
        min={MusicConfig.min}
        max={MusicConfig.max}
        step={MusicConfig.step}
    />
    <div class="p-3 mt-1">
        <div class="grid grid-cols-3">
            <div class="flex items-center">
                <button class="w-10 invert mx-2"
                    ><img
                        class="music-icon"
                        src="/icons/default/previous.png"
                        alt="Icon Previous"
                    /></button
                >
                <button class="w-10 invert mx-2" on:click={handleButtonPlayPause}
                    ><img
                        class="music-icon"
                        src={`/icons/default/${!$musicIsPlaying ? "play" : "pause"}.png`}
                        alt="Icon Play"
                    /></button
                >
                <button class="w-10 invert mx-2"
                    ><img
                        class="music-icon"
                        src="/icons/default/next.png"
                        alt="Icon Next"
                    /></button
                >
            </div>
            <div class="flex items-center justify-center">
                <div class="grid grid-cols-[max-content_auto]">
                    <img class="w-12 rounded" src={albumImage} alt="Album" />
                    <div class="ms-3">
                        <p class="font-medium">
                            {$musicPlayed?.title ?? "The Meaning of Title"}
                        </p>
                        <p class="text-gray-400">
                            {$musicPlayed?.artist ?? "The Artist"}{albumArtist}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
