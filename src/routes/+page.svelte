<script lang="ts">
    import { prominent } from "color.js";
    import "./page.scss";
    
    const SIZE = 10;
    const GRID_COLS = Array.apply(null, Array(SIZE)).map(() => "auto").join(" ");
    console.log(GRID_COLS);
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
    getColors();
    
</script>
<div class="w-screen h-screen absolute grid items-center justify-center">
    <img src="/wild-youth.jpg" class="album">
</div>
<div class="bg-blur">
</div>
<div class="bg-blur-colors" style={`grid-template-columns: ${GRID_COLS}`}>
    {#each position as row}
        {#each row as col}
            <div class="bg-blur-pixel" style={`background: ${col}`}></div>
        {/each}
    {/each}
</div>
