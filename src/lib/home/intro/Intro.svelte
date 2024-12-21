<script lang="ts">
    import LoadingController from "$lib/controllers/LoadingController";
    import MusicController from "$lib/controllers/MusicController";
    import { isMobile } from "$lib/platform";
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

    async function requestReadAudioPermission() {
        const res = await invoke<boolean>("request_read_audio_permission");
        if (!res) return;
        animatedClasses = "animate__fadeOut";
    }

    function onAnimationEnd() {
        if (animatedClasses === "animate__fadeIn") return;
        LoadingController.setLoadingMusicList(false);
        MusicController.getMusics();
    }
</script>

<div
    class={`h-full grid justify-items-center items-center animate__animated ${animatedClasses}`}
    onanimationend={onAnimationEnd}
>
    <div class="grid justify-items-center text-center px-5 lg:px-0">
        {#if isMobile()}
            <p class="font-medium text-2xl">
                Please give us the permission to read the musics.
            </p>
            <p class="mt-3">
                Due to privacy and limtation issues, we will only scan your
                "Music" directory.
            </p>
        {:else}
            <p class="font-medium text-3xl">
                Please pick the folder with your music.
            </p>
        {/if}
        <button
            class="border border-gray-300 bg-gray-500 bg-opacity-10 hover: hover:bg-opacity-20 rounded w-fit mt-5 p-2"
            onclick={() =>
                isMobile() ? requestReadAudioPermission() : chooseDirPath()}
        >
            {isMobile() ? "Show Permission Request" : "Choose Your Folder"}
        </button>
    </div>
</div>
