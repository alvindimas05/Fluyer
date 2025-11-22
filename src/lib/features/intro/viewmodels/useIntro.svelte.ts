import { isAndroid, isDesktop } from "$lib/platform";
import TauriIntroAPI from "$lib/tauri/TauriIntroAPI";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import loadingStore from "$lib/stores/loading.svelte";

export function useIntro() {
    let animatedClasses = $state("animate__fadeIn");

    async function requestAction() {
        if (isAndroid()) {
            const result = await TauriIntroAPI.requestReadAudioPermission();
            if (!result) return;
        }
        await requestDirectoryPath();
    }

    async function requestDirectoryPath() {
        const path = await TauriIntroAPI.requestDirectoryPath();
        if (isAndroid()) await PersistentStoreService.musicPath.set([path]);

        animatedClasses = "animate__fadeOut";
    }

    function onAnimationEnd(currentClass: string) {
        if (currentClass === "animate__fadeIn") return;
        loadingStore.musicList = false;
    }

    return {
        animatedClasses,
        requestAction,
        onAnimationEnd,
    };
}
