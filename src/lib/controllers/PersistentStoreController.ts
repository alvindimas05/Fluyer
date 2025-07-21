import { Store } from "@tauri-apps/plugin-store";
import {
	settingAnimatedBackgroundType,
	settingDeveloperMode,
	settingUiPlayShowBackButton,
	settingUiShowRepeatButton,
	settingUiShowShuffleButton
} from "$lib/stores/setting";
import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";
import { IconThemeType } from "$lib/icon/types";
import { iconTheme } from "$lib/stores/icon";

const storePath = "store.json";
const storeOptions = { autoSave: true };

const getStore = async () => Store.load(storePath, storeOptions);
const makeGetter = <T>(key: string, fallback?: T) => async () =>
	(await (await getStore()).get<T>(key)) ?? fallback;
const makeSetter = <T>(key: string) => async (value: T) =>
	(await getStore()).set(key, value);
const makeBinding = <T>(key: string, fallback: T, setStoreFn: (v: T) => void) => ({
	initialize: async () => setStoreFn(await makeGetter<T>(key, fallback)()),
	get: makeGetter<T>(key, fallback),
	set: makeSetter<T>(key),
});

const PersistentStoreController = {
	get: getStore,

	initialize: async () => {
		await Promise.all([
			PersistentStoreController.animatedBackgroundType.initialize(),
			PersistentStoreController.iconTheme.initialize(),
			PersistentStoreController.developerMode.initialize(),
			PersistentStoreController.userInterface.play.showBackButton.initialize(),
			PersistentStoreController.userInterface.showRepeatButton.initialize(),
			PersistentStoreController.userInterface.showShuffleButton.initialize(),
		]);
	},

	musicPath: {
		key: "music-path",
		separator: "||",
		get: async () =>
			(await makeGetter<string>("music-path")())?.split("||") ?? [],
		set: async (value: string[]) =>
			(await getStore()).set("music-path", value.join("||")),
		add: async (value: string) => {
			const paths = await PersistentStoreController.musicPath.get();
			paths.push(value);
			await PersistentStoreController.musicPath.set(paths);
		},
		remove: async (index: number) => {
			const paths = await PersistentStoreController.musicPath.get();
			paths.splice(index, 1);
			await PersistentStoreController.musicPath.set(paths);
		},
	},

	animatedBackgroundType: makeBinding(
		"animated-background-type",
		SettingAnimatedBackgroundType.Prominent,
		settingAnimatedBackgroundType.set
	),

	developerMode: makeBinding("developer-mode", false, settingDeveloperMode.set),

	iconTheme: makeBinding("icon-theme", IconThemeType.Phosphor, iconTheme.set),

	swipeGuide: {
		get: makeGetter<boolean>("swipe-guide", true),
		set: makeSetter<boolean>("swipe-guide"),
	},

	userInterface: {
		showRepeatButton: makeBinding(
			"ui-show-repeat-button",
			true,
			settingUiShowRepeatButton.set
		),
		showShuffleButton: makeBinding(
			"ui-show-shuffle-button",
			true,
			settingUiShowShuffleButton.set
		),
		play: {
			showBackButton: makeBinding(
				"ui-play-show-back-button",
				true,
				settingUiPlayShowBackButton.set
			),
		},
	},
};

export default PersistentStoreController;
