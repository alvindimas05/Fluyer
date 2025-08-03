<script lang="ts">
import { CommandRoutes } from "$lib/commands";
import LoadingController from "$lib/controllers/LoadingController";
import MusicController from "$lib/controllers/MusicController";
import { isAndroid, isDesktop, isMobile } from "$lib/platform";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";

let animatedClasses = $state("animate__fadeIn");

async function requestAction() {
	if (isAndroid()) {
		const res = await invoke<boolean>(
			CommandRoutes.REQUEST_READ_AUDIO_PERMISSION,
		);
		if (!res) return;
	}
	await chooseDirPath();
}
async function chooseDirPath() {
	const command = isAndroid()
		? CommandRoutes.ANDROID_REQUEST_DIRECTORY
		: CommandRoutes.MUSIC_REQUEST_DIRECTORY;
	await invoke(command);
	const unlisten = await listen<string>(command, async (e) => {
		if (isAndroid()) await PersistentStoreController.musicPath.set([e.payload]);

		animatedClasses = "animate__fadeOut";
		unlisten();
	});
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
        <p class="font-bold text-2xl md:text-3xl">Let's Set Up Your Music</p>
        <p class="mt-3 text-lg md:text-xl">
            {#if isAndroid()}
                To get started, allow access and select your music folder.
            {:else if isDesktop()}
                Select your music folder to get started.
            {/if}
        </p>
        <button
            class="border border-gray-300 bg-gray-500 bg-opacity-10 hover:bg-opacity-20 rounded w-fit mt-5 p-2"
            onclick={requestAction}>
            {#if isAndroid()}
                Allow Access & Choose Folder
            {:else if isDesktop()}
                Choose Folder
            {/if}
        </button>
    </div>
</div>
