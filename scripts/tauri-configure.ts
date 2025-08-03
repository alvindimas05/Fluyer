import path from "path";
import { spawn } from "promisify-child-process";
import os from "os";
import fs from "fs/promises";
import { exit } from "process";

const mpvSource = path.resolve("src-tauri", "libs");

export async function configure() {
	try {
		await fs.access(path.resolve(".env"));
	} catch {
		await fs.copyFile(path.resolve(".env.example"), path.resolve(".env"));
		console.log(
			"Missing .env! The file is now copied from .env.example and make sure to configure it before re-run.",
		);
		exit();
	}
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

let _env = {
	...process.env,
	FLUYER_MPV_SOURCE: mpvSource,
};
if(os.platform() !== 'win32') _env['PATH'] = `${process.env.PATH}:${mpvSource}`;
if(os.platform() === 'linux') _env['WEBKIT_DISABLE_DMABUF_RENDERER'] = '1';
export const env = _env;