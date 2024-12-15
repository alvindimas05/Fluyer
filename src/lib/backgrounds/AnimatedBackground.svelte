<script lang="ts">
    import MusicController, {
        MusicConfig,
    } from "$lib/controllers/MusicController";
    import { observerCounts } from "$lib/stores/background";
    import { musicCurrent } from "$lib/stores/music";
    import { prominent } from "color.js";
    import "./background.scss";
    import LoadingController from "$lib/controllers/LoadingController";

    const SIZE = 10;

    const GRID_COLS = Array.apply(null, Array(SIZE))
        .map(() => "auto")
        .join(" ");

    let animatedClasses = $state("hidden");
    let lastImage = "";
    let position: string[][] = $state([]);
    let isLight = $state(false);

    function hexToRgb(hex: string): { r: number; g: number; b: number } {
        const bigint = parseInt(hex.slice(1), 16);
        const r = (bigint >> 16) & 255;
        const g = (bigint >> 8) & 255;
        const b = bigint & 255;
        return { r, g, b };
    }

    function getLuminance(r: number, g: number, b: number): number {
        const a = [r, g, b].map((v) => {
            v /= 255;
            return v <= 0.03928
                ? v / 12.92
                : Math.pow((v + 0.055) / 1.055, 2.4);
        });
        return 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2];
    }

    function isMajorityLight(colors: string[]): boolean {
        let lightCount = 0;
        let darkCount = 0;

        for (const hex of colors) {
            const { r, g, b } = hexToRgb(hex);
            const luminance = getLuminance(r, g, b);
            if (luminance > 0.5) {
                lightCount++;
            } else {
                darkCount++;
            }
        }

        return lightCount > darkCount;
    }

    async function getColors() {
        if (lastImage === MusicController.currentMusicAlbumImage()) return;
        $observerCounts = 0;
        position = [];

        let image = new Image();
        image.src = lastImage = MusicController.currentMusicAlbumImage();

        // @ts-ignore
        let colors: Hex[] = await prominent(image, {
            amount: 10,
            format: "hex",
        });
        isLight = isMajorityLight(colors);

        for (var i = 0; i < SIZE; i++) {
            position[i] = [];
        }

        for (let i = 0; i < SIZE; i++) {
            for (let j = 0; j < SIZE; j++) {
                position[i][j] =
                    colors[Math.floor(Math.random() * colors.length)];
            }
        }
        animatedClasses = "animate__animated animate__fadeIn";
    }
    musicCurrent.subscribe(getColors);
</script>

{#if isLight}
    <div class="bg-blur-dark"></div>
{/if}
<div
    class={`fixed ${animatedClasses}`}
    onanimationend={() => LoadingController.setLoadingBackground(true)}
>
    <div class="bg-blur"></div>
    <div class="bg-blur-heart">
        <div
            class="bg-blur-colors"
            style={`grid-template-columns: ${GRID_COLS}`}
        >
            {#each position as row}
                {#each row as col}
                    <div
                        class="bg-blur-pixel"
                        style={`background: ${col}`}
                    ></div>
                {/each}
            {/each}
        </div>
    </div>
    <div>
        <div
            class="bg-blur-colors"
            style={`grid-template-columns: ${GRID_COLS}`}
        >
            {#each position as row}
                {#each row as col}
                    <div
                        class="bg-blur-pixel"
                        style={`background: ${col}`}
                    ></div>
                {/each}
            {/each}
        </div>
    </div>
</div>
