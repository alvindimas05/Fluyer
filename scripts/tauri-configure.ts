import path from "path";
import { spawn } from "promisify-child-process";
import os from "os";
import fs from "fs/promises";
import { exit } from "process";

const source = path.resolve("src-tauri", "libs");

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
    const promises = [installBass(), installFfmpeg()];
    await Promise.all(promises);
}

async function installFfmpeg() {
	await spawn("bun", ["scripts/install-ffmpeg.ts"], { stdio: "inherit" });
}

async function installBass(){
    await spawn("bun", ["scripts/install-bass.ts"], { stdio: "inherit" });
}

let _env = {
	...process.env,
	FLUYER_LIBS_SOURCE: source,
};
_env["PATH"] = `${process.env.PATH}${os.platform() === "win32" ? ";" : ":"}${source}`;
export const env = _env;
