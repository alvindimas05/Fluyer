<script lang="ts">
import { musicList } from "$lib/stores/music";
import MusicItem from "./MusicItem.svelte";
</script>

<div class="px-3 overflow-y-auto scrollbar-hidden pb-20">
    <div
    	class="text-white grid gap-x-3
    	md-mdpi:grid-cols-2 lg-mdpi:grid-cols-3 xl-mdpi:grid-cols-4
		md-hdpi:grid-cols-2 xl-hdpi:grid-cols-3"
        >
    	{#if Array.isArray($musicList)}
    		{#each $musicList as music}
    			<MusicItem {music} />
    		{/each}
    	{/if}
    </div>
</div>
