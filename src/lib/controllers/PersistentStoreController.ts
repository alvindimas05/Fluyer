import { Store } from "@tauri-apps/plugin-store";
import { settingAnimatedBackgroundType } from "$lib/stores/setting";
import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";

const storePath = "store.json";
const storeOptions = { autoSave: true };

const getStore = async () => Store.load(storePath, storeOptions);

const makeGetter = <T>(key: string, fallback?: T) => async () =>
	(await (await getStore()).get<T>(key)) ?? fallback;

const makeSetter = <T>(key: string) => async (value: T) =>
	await (await getStore()).set(key, value);

const PersistentStoreController = {
	initialize: async () => {
		const value = await PersistentStoreController.animatedBackgroundType.get();
		settingAnimatedBackgroundType.set(
			value ?? SettingAnimatedBackgroundType.Prominent,
		);
	},

	get: getStore,

	musicPath: {
		key: "music-path",
		separator: "||",
		get: async () =>
			(await (await getStore()).get<string>("music-path"))?.split("||") ?? [],
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

	animatedBackgroundType: {
		key: "animated-background-type",
		get: makeGetter<SettingAnimatedBackgroundType>("animated-background-type"),
		set: makeSetter<SettingAnimatedBackgroundType>("animated-background-type"),
	},

	swipeGuide: {
		key: "swipe-guide",
		get: makeGetter<boolean>("swipe-guide", true),
		set: makeSetter<boolean>("swipe-guide"),
	},

	userInterface: {
		showRepeatButton: {
			key: "ui-show-repeat-button",
			get: makeGetter<boolean>("ui-show-repeat-button", true),
			set: makeSetter<boolean>("ui-show-repeat-button"),
		},
		showShuffleButton: {
			key: "ui-show-shuffle-button",
			get: makeGetter<boolean>("ui-show-shuffle-button", true),
			set: makeSetter<boolean>("ui-show-shuffle-button"),
		},
		play: {
			showBackButton: {
				key: "ui-play-show-back-button",
				get: makeGetter<boolean>("ui-play-show-back-button", true),
				set: makeSetter<boolean>("ui-play-show-back-button"),
			},
		},
	},
};

export default PersistentStoreController;
