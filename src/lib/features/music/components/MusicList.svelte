<script lang="ts">
import MusicItem from "./MusicItem.svelte";
import { VList } from "virtua/svelte";
import { useMusicList } from "../viewmodels/useMusicList.svelte";
import playerBarStore from "$lib/stores/playerbar.svelte";

const vm = useMusicList();
</script>

<svelte:window onresize={vm.updateSize} />

<div class="h-full px-3">
    {#if vm.data && vm.state.columnCount}
        <VList
                class="scrollbar-hidden"
                data={vm.data}
                getKey={(_, i) => i}
                style="padding-bottom: {playerBarStore.height}px;"
        >
            {#snippet children(list)}
                <div
                        class="grid gap-x-6"
                        style="grid-template-columns: repeat({vm.state.columnCount}, minmax(0, 1fr));"
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
