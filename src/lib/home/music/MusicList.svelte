<script lang="ts">
import { musicList } from "$lib/stores/music";
import MusicItem from "./MusicItem.svelte";
</script>

<div class="px-3 overflow-y-auto scrollbar-hidden pb-20">
    <div
    	class="md:grid md:grid-cols-2 tb:grid-cols-2 lg:grid-cols-3 text-white"
        >
    	{#if Array.isArray($musicList)}
    		{#each $musicList as music}
    			<MusicItem {music} />
    		{/each}
    	{/if}
    </div>
</div>
