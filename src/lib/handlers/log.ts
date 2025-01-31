import { CommandsRoute } from "$lib/commands";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function logHandler() {
    listen("debug", (e) => console.log(e.payload));
	window.addEventListener("error", (e) =>
		invoke(CommandsRoute.LOG_ERROR, { message: e.error.toString() }),
	);
	window.addEventListener("unhandledrejection", (e) =>
		invoke(CommandsRoute.LOG_ERROR, { message: e.reason.toString() }),
	);
}
