<script lang="ts">
    import { musicIsPlaying } from "$lib/stores/music";
    import "./page.scss";
    import { MusicConfig } from "$lib/controllers/MusicController";
</script>

<div class="w-full h-full grid items-center justify-center">
    <div class="max-w-[75rem]">
        <div class="w-full text-white">
            <div class="w-full">
                <img class="rounded-lg w-[40rem]" alt="Music Album" />
            </div>
            <div class="w-full grid grid-cols-3">
                <div class="text-sm flex">
                    <span class="self-end">{null}</span>
                </div>
                <div class="text-xl font-medium text-center mt-2">
                    Dabin - Wild Youth
                </div>
                <div class="text-sm flex justify-end">
                    <span class="self-end">-{null}</span>
                </div>
            </div>
            <div class="w-full mt-[-4px]">
                <input
                    id="music-progress-bar"
                    style={`--progress-width: ${null}%`}
                    class="w-full"
                    min={MusicConfig.min}
                    max={MusicConfig.max}
                    step={MusicConfig.step}
                    type="range"
                />
            </div>
            <div class="w-full grid grid-cols-3 mt-2">
                <div class="flex justify-end">
                    <button class="w-12 btn-music-player"
                        ><img
                            class="music-icon"
                            src="/icons/default/previous.png"
                            alt="Icon Previous"
                        /></button
                    >
                </div>
                <div class="flex justify-center">
                    <button class="w-12 btn-music-player">
                        <img
                            class="music-icon"
                            src={`/icons/default/${!$musicIsPlaying ? "play" : "pause"}.png`}
                            alt="Icon Play"
                        />
                    </button>
                </div>
                <div>
                    <button class="w-12 btn-music-player"
                        ><img
                            class="music-icon"
                            src="/icons/default/next.png"
                            alt="Icon Next"
                        /></button
                    >
                </div>
            </div>
        </div>
    </div>
</div>
