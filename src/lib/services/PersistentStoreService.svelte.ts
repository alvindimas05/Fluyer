import { Store } from "@tauri-apps/plugin-store";
import musicStore from "$lib/stores/music.svelte";
import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";
import settingStore from "$lib/stores/setting.svelte";
import { IconThemeType } from "$lib/icon/types";
import iconStore from "$lib/stores/icon.svelte";
import equalizerStore from "$lib/stores/equalizer.svelte";

const storePath = "store.json";
const storeOptions = { autoSave: true };

const getStore = async () => Store.load(storePath, storeOptions);

type InitTask = () => Promise<void>;

interface PersistedProperty<T> {
    get: () => Promise<T>;
    set: (value: T) => Promise<void>;
}

interface PersistentStoreServiceType {
    _initTasks: InitTask[];
    initialize: () => Promise<void>;
    volume: PersistedProperty<number>;
    animatedBackgroundType: PersistedProperty<SettingAnimatedBackgroundType>;
    developerMode: PersistedProperty<boolean>;
    iconTheme: PersistedProperty<IconThemeType>;
    bitPerfectMode: PersistedProperty<boolean>;
    equalizer: PersistedProperty<number[]>;
    uiShowRepeatButton: PersistedProperty<boolean>;
    uiShowShuffleButton: PersistedProperty<boolean>;
    uiPlayShowBackButton: PersistedProperty<boolean>;
    uiPlayShowVolume: PersistedProperty<boolean>;
    musicPath: {
        get: () => Promise<string[]>;
        set: (list: string[]) => Promise<void>;
        add: (path: string) => Promise<void>;
        remove: (index: number) => Promise<void>;
    };
}

function definePersistedProperty<T>(
    target: Record<string, unknown>,
    key: string,
    storeKey: string,
    fallback: T,
    onLoad?: (value: T) => void
) {
    target[key] = {
        get: async () => {
            const store = await getStore();
            return (await store.get<T>(storeKey)) ?? fallback;
        },
        set: async (value: T) => {
            const store = await getStore();
            await store.set(storeKey, value);
        },
    };

    const initTasks = target._initTasks as InitTask[];
    initTasks.push(async () => {
        const store = await getStore();
        const v = (await store.get<T>(storeKey)) ?? fallback;
        onLoad?.(v);
    });
}

const PersistentStoreService: PersistentStoreServiceType = {
    _initTasks: [],

    initialize: async () => {
        for (const fn of PersistentStoreService._initTasks) {
            await fn();
        }
    },
} as PersistentStoreServiceType;

definePersistedProperty(
    PersistentStoreService,
    "volume",
    "volume",
    1,
    (v) => musicStore.volume = v
);

definePersistedProperty(
    PersistentStoreService,
    "animatedBackgroundType",
    "animated-background-type",
    SettingAnimatedBackgroundType.Prominent,
    (v) => settingStore.animatedBackground.type = v
);

definePersistedProperty(
    PersistentStoreService,
    "developerMode",
    "developer-mode",
    false,
    (v) => settingStore.developerMode = v
);

definePersistedProperty(
    PersistentStoreService,
    "iconTheme",
    "icon-theme",
    IconThemeType.Phosphor,
    (v) => iconStore.theme = v
);

definePersistedProperty(
    PersistentStoreService,
    "bitPerfectMode",
    "bit-perfect-mode",
    false,
    (v) => settingStore.bitPerfectMode = v
);

definePersistedProperty(
    PersistentStoreService,
    "equalizer",
    "equalizer",
    Array(18).fill(0),
    (v) => equalizerStore.values = v
);

definePersistedProperty(
    PersistentStoreService,
    "uiShowRepeatButton",
    "ui-show-repeat-button",
    true,
    (v) => settingStore.ui.showRepeatButton = v
);

definePersistedProperty(
    PersistentStoreService,
    "uiShowShuffleButton",
    "ui-show-shuffle-button",
    true,
    (v) => settingStore.ui.showShuffleButton = v
);

definePersistedProperty(
    PersistentStoreService,
    "uiPlayShowBackButton",
    "ui-play-show-back-button",
    true,
    (v) => settingStore.ui.play.showBackButton = v
);

definePersistedProperty(
    PersistentStoreService,
    "uiPlayShowVolume",
    "ui-play-show-volume",
    true,
    (v) => settingStore.ui.play.showVolume = v
);

PersistentStoreService.musicPath = {
    get: async () => {
        const store = await getStore();
        const raw = await store.get<string>("music-path");
        return raw ? raw.split("||") : [];
    },
    set: async (list: string[]) => {
        const store = await getStore();
        await store.set("music-path", list.join("||"));
    },
    add: async (path: string) => {
        const arr = await PersistentStoreService.musicPath.get();
        arr.push(path);
        await PersistentStoreService.musicPath.set(arr);
    },
    remove: async (index: number) => {
        const arr = await PersistentStoreService.musicPath.get();
        arr.splice(index, 1);
        await PersistentStoreService.musicPath.set(arr);
    },
};

export default PersistentStoreService;