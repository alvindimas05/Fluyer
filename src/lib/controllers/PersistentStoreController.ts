import {Store} from "@tauri-apps/plugin-store";

const PersistentStoreController = {
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
};

export default PersistentStoreController;