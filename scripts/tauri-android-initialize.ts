import * as path from "path";
import fs from "fs/promises";
import { downloadFile } from "./install-helpers";
const version = "6.0";
const filename = "ffmpeg-kit-main.aar";
const destDir = path.resolve(
    "src-tauri",
    "tauri-plugin-fluyer",
    "android",
    "libs",
);
const destPath = path.resolve(destDir, filename);
const downloadUrl = `https://github.com/alvindimas05/ffmpeg-kit/releases/download/v${version}/${filename}`;

try {
    await fs.access(destPath);
    console.log("Ffmpeg for Android is already installed. Reinstalling...");
    await fs.rm(destPath);
} catch { }
try {
    console.log("Installing Ffmpeg for Android...");
    await fs.mkdir(destDir, { recursive: true });
    await downloadFile(downloadUrl, destPath);
} catch (err) {
    console.error("Failed:", err);
}
