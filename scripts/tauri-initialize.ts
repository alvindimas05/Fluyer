import path from "path";
import { spawn } from "promisify-child-process";
import fs from "fs/promises";
import { exit } from "process";

async function installFfmpeg() {
	await spawn("bun", ["scripts/install-ffmpeg.ts"], { stdio: "inherit" });
}

async function installBass() {
	await spawn("bun", ["scripts/install-bass.ts"], { stdio: "inherit" });
}

try {
    await fs.access(path.resolve(".env"));
} catch {
    await fs.copyFile(path.resolve(".env.example"), path.resolve(".env"));
    console.log(
        "Missing .env! The file is now copied from .env.example and make sure to configure it before re-run.",
    );
}
const promises = [installBass(), installFfmpeg()];
await Promise.all(promises);