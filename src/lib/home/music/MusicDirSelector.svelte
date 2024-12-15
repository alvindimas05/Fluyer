<script lang="ts">
    import LoadingController from "$lib/controllers/LoadingController";
    import MusicController from "$lib/controllers/MusicController";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let animatedClasses = $state("animate__fadeIn");
    async function chooseDirPath() {
        await invoke("music_request_dir");
        const unlisten = await listen("music_request_dir", (e) => {
            animatedClasses = "animate__fadeOut";
            unlisten();
        });
    }

    function onAnimationEnd() {
        LoadingController.setLoadingMusicList(false);
        MusicController.getMusics();
    }
</script>

<div
    class={`h-full grid justify-items-center items-center animate__animated ${animatedClasses}`}
    onanimationend={onAnimationEnd}
>
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
