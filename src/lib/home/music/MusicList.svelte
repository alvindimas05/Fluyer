<script lang="ts">
import { musicList } from "$lib/stores/music";
import MusicItem from "./MusicItem.svelte";
import { VList } from "virtua/svelte";
import {onMount} from "svelte";

// Responsive rules: [minWidth, maxDppxExclusive, columns]
const rules = [
	[1280, 2.01, 3],  // xhdpi & ≥1280px → 3 cols
	[1024, 2.01, 3],
	[768,  2.01, 2],
	[1280, 1.01, 3],  // hdpi
	[1024, 1.01, 2],
	[768,  1.01, 2],
	[1280, 1.0, 4],   // mdpi
	[1024, 1.0, 3],
	[768,  1.0, 2]
];

// Reactive derived store for column count
let columnCount = $state(1);
let list;

function updateColumnCount() {
	const w = window.innerWidth;
	const dpi = window.devicePixelRatio;

	for (const [minW, minDppx, cols] of rules) {
		if (w >= minW && dpi >= minDppx){
			columnCount = cols;
			return;
		}
	}
	columnCount = 1;
}

let data = $derived.by(() => {
	if(!Array.isArray($musicList)) return [];
	const result = [];
	for (let i = 0; i < $musicList.length; i += columnCount) {
		result.push($musicList.slice(i, i + columnCount));
	}
	return result;
});

onMount(() => {
	updateColumnCount();
});
</script>

<svelte:window onresize={updateColumnCount} />
<div class="px-3 overflow-y-auto text-white">
<!--    <div-->
<!--    	class="grid gap-x-3-->
<!--    	md-mdpi:grid-cols-2 lg-mdpi:grid-cols-3 xl-mdpi:grid-cols-4-->
<!--		md-hdpi:grid-cols-2 xl-hdpi:grid-cols-3-->
<!--		md-xhdpi:grid-cols-2 lg-xhdpi:grid-cols-3"-->
<!--        >-->
<!--    	{#if Array.isArray($musicList)}-->
<!--    		{#each $musicList as music}-->
<!--    			<MusicItem {music} />-->
<!--    		{/each}-->
<!--    	{/if}-->
<!--    </div>-->
	{#if data && columnCount}
		<VList class="scrollbar-hidden" {data} getKey={(_, i) => i}>
			{#snippet children(musicList)}
				<div class="grid" style="grid-template-columns: repeat({columnCount}, minmax(0, 1fr))">
					{#each musicList as music}
						<MusicItem {music} />
					{/each}
				</div>
			{/snippet}
		</VList>
	{/if}
</div>
