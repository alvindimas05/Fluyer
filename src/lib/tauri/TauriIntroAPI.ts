import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/constants/CommandRoutes";
import {isAndroid} from "$lib/platform";
import {listen} from "@tauri-apps/api/event";

const TauriIntroAPI = {
    requestReadAudioPermission: async () => {
        if(isAndroid()) return false;
        return await invoke<boolean>(
            CommandRoutes.REQUEST_READ_AUDIO_PERMISSION,
        );
    },
    requestDirectoryPath: () => {
        return new Promise<string>(async (resolve, _) => {
            const command = isAndroid()
                ? CommandRoutes.ANDROID_REQUEST_DIRECTORY
                : CommandRoutes.MUSIC_REQUEST_DIRECTORY;

            await invoke(command);

            const unlisten = await listen<string>(command, (e) => {
                resolve(e.payload);
                unlisten();
            });
        });
    },

};

export default TauriIntroAPI;