<script lang="ts">

import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import {open} from "@tauri-apps/plugin-dialog";
import MusicController from "$lib/controllers/MusicController";
import {IconType} from "$lib/icon/types";
import Icon from "$lib/icon/Icon.svelte";
import {settingIsLoading} from "$lib/stores/setting";
import {onMount} from "svelte";
import SettingLabel from "$lib/settings/SettingLabel.svelte";

let musicPath = $state<string[]>([]);
let isLoading = $derived($settingIsLoading);

async function refreshPath(){
    musicPath = await PersistentStoreController.musicPath.get();
}

async function addPath(){
    const newPath = await open({
        directory: true,
        multiple: false,
    });
    if(!newPath || musicPath.includes(newPath)) return;

    isLoading = true;
    await PersistentStoreController.musicPath.add(newPath);
    await refreshPath();
    await MusicController.getMusics(true);
    isLoading = false
}

async function removePath(index: number){
    isLoading = true;
    await PersistentStoreController.musicPath.remove(index);
    await refreshPath();
    await MusicController.getMusics(true);
    isLoading = false;
}

onMount(refreshPath);
</script>

<SettingLabel
    title="Music Library Paths"
    description="Directories where your music files are stored."
/>

{#each musicPath as path, index}
    <div class="w-full bg-white/10 text-white placeholder-white/50 px-3 py-2 my-3 rounded-md
        outline-none focus:ring-2 focus:ring-white/30 transition
        grid grid-cols-[auto_min-content]">
        <input
            class="w-full bg-transparent"
            value={path}
            readonly
        >
        {#if musicPath.length > 1}
            <button class="w-6 h-6 flex items-center justify-center cursor-pointer hover:bg-red-500/20 rounded transition"
                onclick={() => removePath(index)}>
                <Icon type={IconType.Trash} weight="fill" color="rgb(255, 150, 150)" />
            </button>
        {/if}
    </div>
{/each}
<div class="w-full bg-white/10 text-white placeholder-white/50 px-3 py-2 my-3 rounded-md
    outline-none focus:ring-2 focus:ring-white/30 transition
    grid grid-cols-[auto_min-content] cursor-pointer">
    <input
        class="w-full bg-transparent text-gray-300 cursor-pointer"
        value="Add new music path..."
        readonly
        disabled={isLoading}
        onclick={addPath}
    >
</div>

