<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    async function chooseDirPath() {
        await invoke("music_get_dir_path");
        const unlisten = await listen("music_get_dir_path", (e) => {
            console.log(e);
            unlisten()
        });
    }
</script>

<div class="h-full grid justify-items-center items-center">
    <div class="grid justify-items-center text-center">
        <p class="font-medium text-3xl">
            Please pick the folder with your music.
        </p>
        <button
            class="border border-gray-300 bg-gray-500 bg-opacity-10 hover: hover:bg-opacity-20 rounded w-fit mt-5 p-2"
            onclick={chooseDirPath}
        >
            Choose Your Folder
        </button>
    </div>
</div>
