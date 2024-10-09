<script lang="ts">
    import { prominent } from "color.js";
    import "./page.scss";
    
    const SIZE = 10;
    const GRID_COLS = Array.apply(null, Array(SIZE)).map(() => "auto").join(" ");
    const ALBUM_PATH = "/test-album.jpg"
    
    const musicDuration = 248;
    const musicProgressMin = 0;
    const musicProgressMax = 10;
    const musicProgressStep = .01;
    
    let musicProgressInterval: ReturnType<typeof setInterval>;
    let musicProgressValue: number;
    let musicProgressDuration = 0;
    
    $: musicProgressPercentage = ((musicProgressValue - musicProgressMin) / (musicProgressMax - musicProgressMin)) * 100;
    $: musicProgressDuration = (musicProgressValue / musicProgressMax) * musicDuration;
    $: musicParsedDuration = getParsedDuration(musicProgressDuration);
    $: musicParsedDurationNegative = getParsedDuration(musicProgressDuration, true);
    
    let position: string[][] = [];
    async function getColors(){
        // @ts-ignore
        let colors: Hex[] = await prominent(ALBUM_PATH, { amount: 10, format: "hex" });
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
    function startProgress() {
        const updateInterval = (musicDuration / (musicProgressMax / musicProgressStep)) * 1000;
    
        musicProgressInterval = setInterval(() => {
            musicProgressValue = Math.min(musicProgressValue + musicProgressStep, musicProgressMax);

            if (musicProgressValue >= musicProgressMax) {
                clearInterval(musicProgressInterval);
            }
        }, updateInterval);
    }
    function getParsedDuration(duration: number, negative = false): string {
        let minutes = 0;
        let seconds = negative ? musicDuration - duration : duration;
        
        while(seconds > 60){
            minutes++;
            seconds -= 60;
        }
        seconds = Math.floor(seconds);
        return `${minutes}:${seconds < 10 ? 0 : ""}${seconds}`;
    }

    resetMusicProgress()
    getColors();
    startProgress();
</script>
<div class="w-screen h-screen absolute grid items-center justify-center">
    <div class="max-w-[75rem]">
        <div class="w-full text-white">
            <div class="w-full">
                <img src={ALBUM_PATH} class="rounded-lg" alt="Music Album">
            </div>
            <div class="w-full grid grid-cols-3">
                <div class="text-sm flex"><span class="self-end">{musicParsedDuration}</span></div>
                <div class="text-xl text-center mt-2">Dabin - Wild Youth</div>
                <div class="text-sm flex justify-end"><span class="self-end">-{musicParsedDurationNegative}</span></div>
            </div>
            <div class="w-full mt-[-4px]">
                <input bind:value={musicProgressValue} id="music-progress-bar" style={`--progress-width: ${musicProgressPercentage}%`}
                    class="w-full" min={musicProgressMin} max={musicProgressMax} step={musicProgressStep} type="range">
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
