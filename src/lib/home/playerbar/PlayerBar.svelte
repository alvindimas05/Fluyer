<script lang="ts">
    import MusicController, { MusicConfig } from "$lib/Music";
    import "./playerbar.scss";
    import { musicCurrent, musicProgressValue } from "$lib/stores/music";

    let title = $state("");
    let artist = $state("");
    let albumArtist = $state("");
    let albumImage = $state("");

    let isPlaying = $state(MusicController.isPlaying());
    let progressPercentage = $state(MusicController.progressPercentage());

    musicProgressValue.subscribe(updateStates);
    musicCurrent.subscribe(handlePlayPause);

    function handlePlayPause() {
        if (MusicController.isPlaying()) MusicController.play();
        else MusicController.pause();

        updateStates();
        updateProgress();
    }

    function handleButtonPlayPause() {
        if (MusicController.isPlaying()) MusicController.pause();
        else MusicController.play();

        updateStates();
        updateProgress();
    }

    function onPlayerBarChange() {
        if (MusicController.currentMusic() == null) {
            MusicController.setProgressValue(0);
            return;
        }

        if (MusicController.progressValue() == MusicConfig.max) {
            MusicController.addMusic(MusicController.currentMusic()!.path);
            handlePlayPause();
        }

        MusicController.sendCommandSetPosition(
            MusicController.realProgressDuration(),
        );
    }

    function updateProgress() {
        MusicController.stopProgress();
        if (MusicController.isPlaying()) {
            MusicController.startProgress();
        }
    }

    function updateStates() {
        isPlaying = MusicController.isPlaying();
        progressPercentage = MusicController.progressPercentage();

        let music = MusicController.currentMusic();
        if (music == null) return;

        title = music.title!;
        artist = music.artist;
        albumArtist =
            music.album_artist && !music.artist.includes(music.album_artist)
                ? ` • ${music.album_artist}`
                : "";
        albumImage = `data:image/png;base64,${music.image}`;
    }
</script>

<div class="fixed left-0 bottom-0 w-full bg-[rgba(0,0,0,.5)] text-white">
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
        <div class="grid grid-cols-3">
            <div class="flex items-center">
                <button class="w-10 invert mx-2"
                    ><img
                        class="music-icon"
                        src="/icons/default/previous.png"
                        alt="Icon Previous"
                    /></button
                >
                <button
                    class="w-10 invert mx-2"
                    onclick={handleButtonPlayPause}
                    ><img
                        class="music-icon"
                        src={`/icons/default/${!isPlaying ? "play" : "pause"}.png`}
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
                            {title}
                        </p>
                        <p class="text-gray-400">
                            {artist ?? "The Artist"}{albumArtist}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
