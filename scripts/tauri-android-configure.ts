import * as path from "path";
import fs from "fs/promises";
import { downloadFile } from "./install-helpers";
const version = "6.0";
const filename = "ffmpeg-kit-main.aar";
const destPath = path.resolve("src-tauri", "tauri-plugin-fluyer", "android", "libs", filename);
const downloadUrl = `https://github.com/alvindimas05/ffmpeg-kit/releases/download/v${version}/${filename}`;

async function installFfmpeg() {
    try {
        await fs.access(destPath);
        return;
    } catch {}
    try {
        console.log("Installing ffmpeg for Android...");
        await downloadFile(downloadUrl, destPath);
    } catch (err) {
        console.error("Failed:", err);
    }
}

await installFfmpeg();