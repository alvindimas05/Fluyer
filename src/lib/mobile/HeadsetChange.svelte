<script lang="ts">
import { CommandsRoute } from "$lib/commands";
import MusicController from "$lib/controllers/MusicController";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let show = $state(false);
// let seconds = $state(10);
// let interval: ReturnType<typeof setInterval> | null = null;

// function restartA/pp() {
	// invoke("plugin:fluyer|restart_app");
// }

listen(CommandsRoute.MUSIC_HEADSET_CHANGE, () => {
	show = true;
	// interval = setInterval(() => {
	// 	if (seconds > 0) seconds--;
	// 	else {
	// 		clearInterval(interval!);
	// 		restartApp();
	// 	}
	// }, 1000);
});

MusicController.pause();
</script>

{#if show}
    <div
        class="w-screen h-screen grid justify-items-center items-center backdrop-blur-lg"
    >
        <div
            class="grid w-fit h-fit justify-items-center text-center px-3 py-5 m-3 rounded border animate__animated animate__fadeIn"
        >
            <p class="font-medium text-2xl">
                Output Sound Device Change Detected
            </p>
            <p class="mt-2">
                We apologize for the inconvenience. Due to current package
                limitations, a restart is required to restore sound
                functionality.
            </p>
            <button
                class="border border-gray-300 bg-gray-500 bg-opacity-10 hover:bg-opacity-20 rounded w-fit mt-5 p-2"
                onclick={restartApp}
            >
                Restart Now
            </button>
        </div>
    </div>
{/if}
