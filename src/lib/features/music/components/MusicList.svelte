<script lang="ts">
import MusicItem from "./MusicItem.svelte";
import { VList } from "virtua/svelte";
import { playerBarHeight } from "$lib/stores/playerbar.svelte";
import { useMusicList } from "../viewmodels/useMusicList.svelte";

const vm = useMusicList();
</script>

<svelte:window on:resize={vm.updateSize} />

<div class="h-full px-3">
    {#if vm.data && vm.columnCount}
        <VList
                class="scrollbar-hidden"
                data={vm.data}
                getKey={(_, i) => i}
                style="padding-bottom: {$playerBarHeight}px;"
        >
            {#snippet children(list)}
                <div
                        class="grid gap-x-6"
                        style="grid-template-columns: repeat({vm.columnCount}, minmax(0, 1fr));"
                >
                    {#each list as item}
                        {#if 'duration' in item}
                            <MusicItem music={item}/>
                        {:else}
                            <MusicItem folder={item}/>
                        {/if}
                    {/each}
                </div>
            {/snippet}
        </VList>
    {/if}
</div>
