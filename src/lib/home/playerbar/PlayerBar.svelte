<script lang="ts">
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import "./playerbar.scss";
import {
	musicCurrent,
	musicIsPlaying,
	musicProgressValue,
} from "$lib/stores/music";

let title = $state(MusicConfig.defaultTitle);
let artist = $state(MusicConfig.defaultArtist);
let albumImage = $state(MusicConfig.defaultAlbumImage);

let isPlaying = $state(MusicController.isPlaying());
let progressPercentage = $state(MusicController.progressPercentage());

musicProgressValue.subscribe(updateStates);
musicCurrent.subscribe(updateStates);

function handleButtonPlayPause() {
	if (MusicController.currentMusic() == null) return;

	if (MusicController.isPlaying()) MusicController.pause();
	else MusicController.play();

	updateStates();
}

function handleButtonNext() {
	MusicController.tryNextMusic(true);
}

function onPlayerBarChange() {
	if (MusicController.currentMusic() == null) {
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
</script>

<div
    class="fixed left-0 bottom-0 z-10 w-full bg-gray-700 bg-opacity-30 backdrop-blur-md text-white animate__animated animate__slideInUp animate__slow"
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
        <div class="grid grid-cols-3">
            <div class="flex items-center">
                <button class="w-10 invert mx-2"
                    ><img
                        class="music-icon"
                        src="/icons/default/previous.png"
                        alt="Icon Previous"
                    /></button
                >
                <button class="w-10 invert mx-2" onclick={handleButtonPlayPause}
                    ><img
                        class="music-icon"
                        src={isPlaying
                            ? MusicConfig.defaultPauseButton
                            : MusicConfig.defaultPlayButton}
                        alt="Icon Play"
                    /></button
                >
                <button class="w-10 invert mx-2" onclick={handleButtonNext}
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
                        <p class="text-gray-200">
                            {artist}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
