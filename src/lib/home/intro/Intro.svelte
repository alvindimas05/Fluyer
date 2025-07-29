<script lang="ts">
import { CommandRoutes } from "$lib/commands";
import LoadingController from "$lib/controllers/LoadingController";
import MusicController from "$lib/controllers/MusicController";
import {isAndroid, isDesktop, isMobile} from "$lib/platform";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";

let animatedClasses = $state("animate__fadeIn");
async function chooseDirPath() {
    const command = isAndroid() ? CommandRoutes.ANDROID_REQUEST_DIRECTORY : CommandRoutes.MUSIC_REQUEST_DIRECTORY
    await invoke(command);
    const unlisten = await listen<string>(command, async (e) => {
        if(isAndroid()) await PersistentStoreController.musicPath.set([e.payload]);

        animatedClasses = "animate__fadeOut";
        unlisten();
    });
}

// async function requestReadAudioPermission() {
// 	const res = await invoke<boolean>(
// 		CommandRoutes.REQUEST_READ_AUDIO_PERMISSION,
// 	);
// 	if (!res) return;
// 	animatedClasses = "animate__fadeOut";
// }

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
        <p class="font-medium text-3xl">
            Please pick the folder with your music.
            </p>
        <button
            class="border border-gray-300 bg-gray-500 bg-opacity-10 hover: hover:bg-opacity-20 rounded w-fit mt-5 p-2"
            onclick={chooseDirPath}>Choose Your Folder</button>
    </div>
</div>
