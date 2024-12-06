<script lang="ts">
    import { album } from "$lib/stores";
    import { prominent } from "color.js";

    export let classes = "";
    const SIZE = 10;

    const GRID_COLS = Array.apply(null, Array(SIZE))
        .map(() => "auto")
        .join(" ");

    let position: string[][] = [];
    async function getColors() {
        if ($album == null) return;
        position = [];

        let image = new Image();
        image.src = $album;
        // @ts-ignore
        let colors: Hex[] = await prominent(image, {
            amount: 10,
            format: "hex",
        });
        for (var i = 0; i < SIZE; i++) {
            position[i] = [];
        }

        for (let i = 0; i < SIZE; i++) {
            for (let j = 0; j < SIZE; j++) {
                position[i][j] =
                    colors[Math.floor(Math.random() * colors.length)];
            }
        }
    }
    $: $album, getColors();
</script>

<div
    class={`bg-blur-colors ${classes}`}
    style={`grid-template-columns: ${GRID_COLS}`}
>
    {#each position as row}
        {#each row as col}
            <div class="bg-blur-pixel" style={`background: ${col}`}></div>
        {/each}
    {/each}
</div>
