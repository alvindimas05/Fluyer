<script lang="ts">
import MusicController from "$lib/controllers/MusicController";
import { musicList } from "$lib/stores/music";
import { platform } from "@tauri-apps/plugin-os";
import MusicItem from "./MusicItem.svelte";
import type { MusicData } from "./types";

function splitMusics(arr: MusicData[] | null | undefined): MusicData[][] {
	if (!Array.isArray(arr)) return [];
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
let splittedMusics = $derived(splitMusics($musicList));
</script>

<div
	class="md:grid md:grid-cols-2 tb:grid-cols-2 lg:grid-cols-3 h-fit text-white px-3 overflow-y-auto scrollbar-hidden pb-20"
>
	{#if Array.isArray($musicList)}
		{#each $musicList as music}
			<MusicItem {music} />
		{/each}
	{:else}
		{#each splittedMusics as musics}
			<div>
				{#each musics as music}
					<MusicItem {music} />
				{/each}
			</div>
		{/each}
	{/if}
</div>
