<script lang="ts">
import PersistentStoreController from "$lib/controllers/PersistentStoreController.js";
import {onMount} from "svelte";
import {open} from '@tauri-apps/plugin-dialog';
import PageController from "$lib/controllers/PageController";
import {PageRoutes} from "$lib/pages";

let musicPath = $state<string[]>([]);

onMount(async () => {
    refreshPath();
});

async function refreshPath(){
    musicPath = await PersistentStoreController.musicPath.get();
}

async function addPath(){
    const newPath = await open({
        directory: true,
        multiple: false,
    });
    if(!newPath) return;
    await PersistentStoreController.musicPath.add(newPath);
    refreshPath();
}

async function removePath(index: number){
    await PersistentStoreController.musicPath.remove(index);
    refreshPath();
}
</script>

<div class="w-full h-full px-4 pb-4 pt-8">
    <div class="w-full h-full p-6 bg-gray-700 bg-opacity-30 rounded-xl shadow-xl">
        <div class="w-full h-full grid grid-rows-[auto_min-content] rounded-lg border border-white/20 p-4 bg-white/5">
            <div>
                <p class="text-2xl font-semibold text-white mb-4">Settings</p>
                <p class="font-semibold">Music Paths</p>
                <p>The paths of your musics.</p>
                {#each musicPath as path, index}
                    <div class="w-full flex items-center gap-3 my-3">
                        <input
                                class="flex-1 bg-white/10 text-white placeholder-white/50 px-3 py-2 rounded-md outline-none focus:ring-2 focus:ring-white/30 transition"
                                value={path}
                                placeholder="Add new music path..."
                                readonly
                        >
                        <button
                                class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-md transition shadow"
                                onclick={() => removePath(index)}
                        >
                            Delete
                        </button>
                    </div>
                {/each}
                <div class="w-full flex items-center gap-3 my-3">
                    <input
                            class="flex-1 bg-white/10 text-white placeholder-white/50 px-3 py-2 rounded-md outline-none focus:ring-2 focus:ring-white/30 transition"
                            placeholder="Add new music path..."
                            readonly
                            onclick={addPath}
                    >
                </div>
            </div>
            <div>
                <button
                        class="rounded text-white text-start font-medium tracking-wide px-4 py-2 bg-gradient-to-r from-white/10 to-white/5 hover:from-white/20 hover:to-white/10 focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200 shadow-md hover:shadow-lg backdrop-blur-md"
                        onclick={() => PageController.back()}
                >
                    Back
                </button>
            </div>
        </div>
    </div>
</div>
