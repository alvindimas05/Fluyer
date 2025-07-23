import { CommandRoutes } from "$lib/commands";
import { invoke } from "@tauri-apps/api/core";
import ToastController from "$lib/controllers/ToastController";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";

export default function logHandler() {
	window.addEventListener("error", async (e) => {
		invoke(CommandRoutes.LOG_ERROR, { message: e.message.toString() });
		if(await PersistentStoreController.developerMode.get()) ToastController.error(e.message.toString());
	});
	window.addEventListener("unhandledrejection", async (e) => {
		const message = e.reason.toString();
		invoke(CommandRoutes.LOG_ERROR, { message });
		if(await PersistentStoreController.developerMode.get()) ToastController.error(message);
	});
}
