import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/constants/CommandRoutes";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import ToastService from "$lib/services/ToastService.svelte";

const LogService = {
    initialize: async () => {
        LogService.listenLog();
        LogService.listenError();
    },
    listenLog: () => {
        const handler = {
            apply(target: any, thisArg: any, args: any) {
                const message = args
                    .map((arg: any) => (typeof arg === "object" && arg !== null ? JSON.stringify(arg) : String(arg)))
                    .join(" ");
                invoke(CommandRoutes.LOG_INFO, { message });
                return Reflect.apply(target, thisArg, args);
            }
        };

        console.log = new Proxy(console.log, handler);

    },
    listenError: () => {
        window.addEventListener("error", async (e) => {
            if (e.message.toString().includes("ResizeObserver")) return;
            invoke(CommandRoutes.LOG_ERROR, { message: e.message.toString() });
            if (await PersistentStoreService.developerMode.get())
                ToastService.error(e.message.toString());
        });
        window.addEventListener("unhandledrejection", async (e) => {
            const message = e.reason.toString();
            invoke(CommandRoutes.LOG_ERROR, { message });
            if (await PersistentStoreService.developerMode.get())
                ToastService.error(message);
        });
    }
};

export default LogService;