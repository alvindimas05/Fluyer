import { invoke } from "@tauri-apps/api/core";

export default function logHandler() {
	window.addEventListener("error", (e) =>
		invoke("log_error", { message: e.error.toString() }),
	);
	window.addEventListener("unhandledrejection", (e) =>
		invoke("log_error", { message: e.reason.toString() }),
	);
}
