import { CommandsRoute } from "$lib/commands";
import { invoke } from "@tauri-apps/api/core";

export default function logHandler() {
	window.addEventListener("error", (e) =>
		invoke(CommandsRoute.LOG_ERROR, { message: e.error.toString() }),
	);
	window.addEventListener("unhandledrejection", (e) =>
		invoke(CommandsRoute.LOG_ERROR, { message: e.reason.toString() }),
	);
}
