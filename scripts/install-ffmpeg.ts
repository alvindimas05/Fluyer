import axios from "axios";
import * as path from "path";
import fs from "fs/promises";
import os from "os";
import { downloadFile, extractTarGz } from "./install-helpers";
const owner = "alvindimas05";
const repo = "ffmpeg-build";
const downloadOutputDir = path.resolve("src-tauri", "libs");
const outputDir = path.resolve(downloadOutputDir, "ffmpeg");

function getLibName() {
    let versionName: string;
    switch(os.platform()) {
        case "win32":
            versionName = "x86_64-w64-mingw32";
            break;
        case "darwin":
            versionName = process.arch === "arm64" ? "arm64-apple-macos11" : "x86_64-apple-macos10.9";
            break;
        case "linux":
            versionName = "x86_64-linux-gnu";
            break;
        default:
            throw new Error("Unsupported platform");
    }
    return `ffmpeg-7.1-audio-${versionName}`;
}
async function getLatestAssetUrl(): Promise<string> {
	const apiUrl = `https://api.github.com/repos/${owner}/${repo}/releases`;
	const res = await axios.get(apiUrl, {
		headers: { Accept: "application/vnd.github+json" },
	});

	const releases = res.data;
	if (!Array.isArray(releases) || releases.length === 0) {
		throw new Error("No releases found.");
	}

	const latestRelease = releases[0];
	const asset = latestRelease.assets.find(
		(a: any) => a.name === getLibName() + ".tar.gz",
	);

	if (!asset) {
		throw new Error("No valid asset found in latest release.");
	}

	return asset.browser_download_url;
}

async function main() {
	try {
		await fs.access(outputDir);
		return;
	} catch {}
	try {
		console.log("Installing ffmpeg...");
		const url = await getLatestAssetUrl();
		const fileName = path.basename(url);
		const destPath = path.join(downloadOutputDir, fileName);

		await downloadFile(url, destPath);
		await extractTarGz(destPath, downloadOutputDir);

        console.log(path.resolve(downloadOutputDir, getLibName()));
        fs.rename(
            path.resolve(downloadOutputDir, getLibName()),
            outputDir,
        );
	} catch (err) {
		console.error("Failed:", err);
	}
}

main();
