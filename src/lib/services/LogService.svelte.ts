import { invoke } from "@tauri-apps/api/core";
import { CommandRoutes } from "$lib/constants/CommandRoutes";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import ToastService from "$lib/services/ToastService.svelte";
import { listen } from "@tauri-apps/api/event";

const LogService = {
    initialize: async () => {
        LogService.listenLog();
        LogService.listenError();
        LogService.listenBackendLog();
    },
    listenLog: () => {
        console.log = new Proxy(console.log, {
            apply(target, thisArg, args) {
                return Reflect.apply(target, thisArg, ["[LOG]", ...args]);
            }
        });
        console.debug = new Proxy(console.debug, {
            apply(target, thisArg, args) {
                return Reflect.apply(target, thisArg, ["[DEBUG]", ...args]);
            }
        });
        console.info = new Proxy(console.info, {
            apply(target, thisArg, args) {
                return Reflect.apply(target, thisArg, ["[INFO]", ...args]);
            }
        });
        console.warn = new Proxy(console.warn, {
            apply(target, thisArg, args) {
                return Reflect.apply(target, thisArg, ["[WARN]", ...args]);
            }
        });
        console.error = new Proxy(console.error, {
            apply(target, thisArg, args) {
                return Reflect.apply(target, thisArg, ["[ERROR]", ...args]);
            }
        });
    },
    listenBackendLog: () => {
        listen<string>(CommandRoutes.LOG, (event) => {
            if (event.payload.startsWith("[ERROR]")) {
                console.error(event.payload);
            } else if (event.payload.startsWith("[WARN]")) {
                console.warn(event.payload);
            } else if (event.payload.startsWith("[INFO]")) {
                console.info(event.payload);
            } else if (event.payload.startsWith("[DEBUG]")) {
                console.debug(event.payload);
            } else {
                console.log(event.payload);
            }
        });
    },
    listenError: () => {
        window.addEventListener("error", async (e) => {
            if (e.message.toString().includes("ResizeObserver")) return;
            // invoke(CommandRoutes.LOG_ERROR, { message: e.message.toString() });
            if (await PersistentStoreService.developerMode.get())
                ToastService.error(e.message.toString());
        });
        window.addEventListener("unhandledrejection", async (e) => {
            const message = e.reason.toString();
            // invoke(CommandRoutes.LOG_ERROR, { message });
            if (await PersistentStoreService.developerMode.get())
                ToastService.error(message);
        });
    }
};

export default LogService;