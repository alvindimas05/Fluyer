<script lang="ts">
import MusicController from "$lib/controllers/MusicController";
import MusicItem from "./MusicItem.svelte";
import type { MusicData } from "./types";

function splitMusics(arr: MusicData[]): MusicData[][] {
	const length = arr.length;

	// Calculate the sizes of the three parts
	const part1Size = Math.ceil(length / 3); // First part size
	const part2Size = Math.ceil((length - part1Size) / 2); // Second part size
	const part3Size = length - part1Size - part2Size; // Remaining elements for the third part

	// Split the array
	const firstPart = arr.slice(0, part1Size);
	const secondPart = arr.slice(part1Size, part1Size + part2Size);
	const thirdPart = arr.slice(part1Size + part2Size);

	return [firstPart, secondPart, thirdPart];
}
let splittedMusics = $derived(splitMusics(MusicController.musicList()!));
</script>

<div class="grid grid-cols-3 text-white px-3 overflow-y-auto scrollbar-hidden pb-20">
    {#each splittedMusics as musics}
        <div>
            {#each musics as music}
                <MusicItem {music} />
            {/each}
        </div>
    {/each}
</div>