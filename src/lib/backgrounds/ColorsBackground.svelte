<script lang="ts">
    import MusicController from "$lib/Music";
    import { musicCurrent } from "$lib/stores/music";
    import { prominent } from "color.js";

    interface Props {
        classes?: string;
    }

    let { classes = "" }: Props = $props();
    const SIZE = 10;

    const GRID_COLS = Array.apply(null, Array(SIZE))
        .map(() => "auto")
        .join(" ");

    let position: string[][] = $state([]);
    async function getColors() {
        if (MusicController.currentMusic() == null) return;
        position = [];

        let image = new Image();
        image.src = `data:image/png;base64,${MusicController.currentMusic()!.image}`;

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
    musicCurrent.subscribe(getColors);
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
