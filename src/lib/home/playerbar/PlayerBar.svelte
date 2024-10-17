<script lang="ts">
    import { MusicConfig } from "$lib/Music";
    import './playerbar.scss'

    let musicValue = 0;
    let musicProgressPercentage = 0;
    $: musicValue, musicProgressPercentage = MusicConfig.getProgressPercentage(musicValue);
</script>

<div class="fixed left-0 bottom-0 w-full bg-[rgba(0,0,0,.9)] text-white">
    <input id="music-progress-bar" class="w-full absolute" type="range" style={`--progress-width: ${musicProgressPercentage}%`} bind:value={musicValue}
        min={MusicConfig.min} max={MusicConfig.max} step={MusicConfig.step}>
    <div class="p-3 mt-1">
        <div class="grid grid-cols-3">
            <div class="flex items-center">
                <button class="w-10 invert mx-2"><img class="music-icon" src="/icons/default/previous.png" alt="Icon Previous"></button>
                <button class="w-10 invert mx-2"><img class="music-icon" src="/icons/default/play.png" alt="Icon Play"></button>
                <button class="w-10 invert mx-2"><img class="music-icon" src="/icons/default/next.png" alt="Icon Next"></button>
            </div>
            <div class="flex items-center justify-center">
                <div class="grid grid-cols-[max-content_auto]">
                    <img class="w-12 rounded" src="/test-album.jpg" alt="Album">
                    <div class="ms-3">
                        <p class="font-medium">Tie Me Down</p>
                        <p class="text-gray-400">Gravity • Gryffin • 2019</p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>