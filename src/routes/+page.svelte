<script lang="ts">
    import { prominent } from "color.js";
    import "./page.scss";
    
    const SIZE = 10;
    const GRID_COLS = Array.apply(null, Array(SIZE)).map(() => "auto").join(" ");
    
    const musicProgressMin = 0;
    const musicProgressMax = 10;
    let musicProgressValue: number;
    $: musicProgressPercentage = ((musicProgressValue - musicProgressMin) / (musicProgressMax - musicProgressMin)) * 100;
    
    let position: string[][] = [];
    async function getColors(){
        // @ts-ignore
        let colors: Hex[] = await prominent("/wild-youth.jpg", { amount: 10, format: "hex" });
        for (var i = 0; i < SIZE; i++) {
            position[i] = [];
        }
    
        for (let i = 0; i < SIZE; i++) {
            for (let j = 0; j < SIZE; j++) {
                position[i][j] = colors[Math.floor(Math.random() * colors.length)];
            }
        }
    }
    function resetMusicProgress(){
        musicProgressValue = musicProgressMin;
    }
    resetMusicProgress()
    getColors();
</script>
<div class="w-screen h-screen absolute grid items-center justify-center">
    <div class="max-w-[75rem]">
        <div class="w-full text-white">
            <div class="w-full">
                <img src="/wild-youth.jpg" class="rounded-lg" alt="Music Album">
            </div>
            <div class="w-full grid grid-cols-3">
                <div class="text-sm flex"><span class="self-end">0:10</span></div>
                <div class="text-xl text-center mt-2">Dabin - Wild Youth</div>
                <div class="text-sm flex justify-end"><span class="self-end">-0:30</span></div>
            </div>
            <div class="w-full mt-[-4px]">
                <input bind:value={musicProgressValue} id="music-progress-bar" style={`--progress-width: ${musicProgressPercentage}%`}
                    class="w-full" min={musicProgressMin} max={musicProgressMax} step="0.01" type="range">
            </div>
            <div class="w-full grid grid-cols-3 mt-2">
                <div class="flex justify-end">
                    <button class="w-12"><img class="music-icon" src="/icons/default/previous.png"></button>
                </div>
                <div class="flex justify-center">
                    <button class="w-12"><img class="music-icon" src="/icons/default/play.png"></button>
                </div>
                <div>
                    <button class="w-12"><img class="music-icon" src="/icons/default/next.png"></button>
                </div>
            </div>
        </div>
    </div>
</div>
<div class="bg-blur"></div>
<div class="bg-blur-colors" style={`grid-template-columns: ${GRID_COLS}`}>
    {#each position as row}
        {#each row as col}
            <div class="bg-blur-pixel" style={`background: ${col}`}></div>
        {/each}
    {/each}
</div>
