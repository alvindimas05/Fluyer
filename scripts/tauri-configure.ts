import path from "path";
import { spawn } from "promisify-child-process";
import os from "os";

const mpvSource = path.resolve("src-tauri", "libs");

export async function configure() {
	switch (os.platform()) {
		case "win32":
			await configureWindows();
			break;
		case "darwin":
			await configureMacos();
			break;
	}
}

async function configureWindows() {
	await spawn("bun", ["scripts/install-libmpv-windows.ts"], {
		stdio: "inherit",
	});
}

async function configureMacos() {
	await spawn("bun", ["scripts/install-libs-macos.ts"], { stdio: "inherit" });
}

export const env = {
	...process.env,
	PATH: `${process.env.PATH};${mpvSource}`,
	FLUYER_MPV_SOURCE: mpvSource,
};
