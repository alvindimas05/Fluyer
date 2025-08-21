import { writable } from "svelte/store";
import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";

export const settingIsLoading = writable(false);
export const settingTriggerAnimatedBackground = writable("");
export const settingAnimatedBackgroundType = writable(
	SettingAnimatedBackgroundType.Pallete,
);
export const settingUiShowRepeatButton = writable(true);
export const settingUiShowShuffleButton = writable(true);
export const settingUiPlayShowBackButton = writable(true);
export const settingUiPlayShowVolume = writable(true);
export const settingDeveloperMode = writable(false);
