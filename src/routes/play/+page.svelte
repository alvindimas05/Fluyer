<script lang="ts">
    import Music from "$lib/Music";
    import { invoke } from "@tauri-apps/api/core";
    import { album } from "$lib/stores";
    import "./page.scss";
    
    let music = new Music();
    let musicIsPlaying = music.isPlaying;

    let musicValue = music.value;
    let musicProgressDuration = music.progressDuration;
    let musicProgressPercentage = music.progressPercentage;
    let musicParsedDuration = music.getParsedDuration();
    let musicParsedDurationNegative = music.getParsedDuration(true);
    
    $: musicValue, (() => { 
        music.value = musicValue;
        updateStates();
    })();
    
    function handlePlayPause() {
        music.playOrPause();
        updateProgress();
        musicIsPlaying = music.isPlaying;
    }
    
    function updateProgress(){
        music.stopProgress();
        if(music.isPlaying){
            music.startProgress(() => {
                musicValue = music.value;
                updateStates();
            });
        }
    }
    
    function updateStates(){
        musicProgressPercentage = music.progressPercentage;
        musicProgressDuration = music.progressDuration;
        musicParsedDuration = music.getParsedDuration();
        musicParsedDurationNegative = music.getParsedDuration(true);
    }

    music.resetProgress();
</script>

<div class="w-full h-full grid items-center justify-center">
    <div class="max-w-[75rem]">
        <div class="w-full text-white">
            <div class="w-full">
                <img src={$album} class="rounded-lg w-[40rem]" alt="Music Album">
            </div>
            <div class="w-full grid grid-cols-3">
                <div class="text-sm flex"><span class="self-end">{musicParsedDuration}</span></div>
                <div class="text-xl font-medium text-center mt-2">Dabin - Wild Youth</div>
                <div class="text-sm flex justify-end"><span class="self-end">-{musicParsedDurationNegative}</span></div>
            </div>
            <div class="w-full mt-[-4px]">
                <input bind:value={musicValue} id="music-progress-bar" style={`--progress-width: ${musicProgressPercentage}%`}
                    class="w-full" min={music.min} max={music.max} step={music.step} type="range">
            </div>
            <div class="w-full grid grid-cols-3 mt-2">
                <div class="flex justify-end">
                    <button class="w-12 btn-music-player"><img class="music-icon" src="/icons/default/previous.png" alt="Icon Previous"></button>
                </div>
                <div class="flex justify-center">
                    <button class="w-12 btn-music-player" on:click={handlePlayPause}>
                        <img class="music-icon" src={`/icons/default/${!musicIsPlaying ? "play" : "pause"}.png`} alt="Icon Play">
                    </button>
                </div>
                <div>
                    <button class="w-12 btn-music-player"><img class="music-icon" src="/icons/default/next.png" alt="Icon Next"></button>
                </div>
            </div>
        </div>
    </div>
</div>
