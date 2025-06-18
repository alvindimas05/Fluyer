import {Store} from "@tauri-apps/plugin-store";
import {settingAnimatedBackgroundType} from "$lib/stores/setting";
import type {SettingAnimatedBackgroundType} from "$lib/settings/animated-background/types";

const PersistentStoreController = {
    initialize: async () => {
        await PersistentStoreController.animatedBackgroundType.setStore();
    },
    get: () => Store.load("store.json", { autoSave: false }),
    musicPath: {
        key: 'music-path',
        separator: '||',
        get: async () => {
            return (await (await PersistentStoreController.get()).get<string>(
                PersistentStoreController.musicPath.key
            ))
                ?.split(PersistentStoreController.musicPath.separator) ?? [];
        },
        set: async (value: string[]) => {
            await (await PersistentStoreController.get())
                .set(
                    PersistentStoreController.musicPath.key,
                    value.join(PersistentStoreController.musicPath.separator)
                );
        },
        add: async (value: string) => {
            let paths = await PersistentStoreController.musicPath.get();
            paths.push(value);
            await PersistentStoreController.musicPath.set(paths);
        },
        remove: async (index: number) => {
            const paths = await PersistentStoreController.musicPath.get()
            paths.splice(index, 1);
            await PersistentStoreController.musicPath.set(paths);
        },
    },
    animatedBackgroundType: {
        key: 'animated-background-type',
        setStore: async () => {
            settingAnimatedBackgroundType.set(
                await PersistentStoreController.animatedBackgroundType.get()
            );
        },
        get: async () => {
            return <SettingAnimatedBackgroundType> await (await PersistentStoreController.get()).get<string>(
                PersistentStoreController.animatedBackgroundType.key
            );
        },
        set: async (value: SettingAnimatedBackgroundType) => {
            await (await PersistentStoreController.get())
                .set(PersistentStoreController.animatedBackgroundType.key, value.toString());
        },
    },
};

export default PersistentStoreController;