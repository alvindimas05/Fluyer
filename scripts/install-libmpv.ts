import axios from "axios";
import * as path from "path";
import { pipeline } from "stream/promises";
import Seven from "node-7z";
import fs from "fs/promises";
import { createWriteStream } from "fs";

const owner = "shinchiro";
const repo = "mpv-winbuild-cmake";
const outputDir = path.resolve("src-tauri", "resources");

async function getLatestAssetUrl(): Promise<string> {
  const apiUrl = `https://api.github.com/repos/${owner}/${repo}/releases`;
  const res = await axios.get(apiUrl, {
    headers: { "Accept": "application/vnd.github+json" },
  });

  const releases = res.data;
  if (!Array.isArray(releases) || releases.length === 0) {
    throw new Error("No releases found.");
  }

  const latestRelease = releases[0];
  const asset = latestRelease.assets.find((a: any) =>
    a.name.endsWith(".7z") && a.name.includes("mpv-dev-x86_64")
  );

  if (!asset) {
    throw new Error("No valid asset found in latest release.");
  }

  return asset.browser_download_url;
}

async function downloadFile(url: string, destPath: string): Promise<void> {
  const response = await axios.get(url, { responseType: "stream" });
  await pipeline(response.data, createWriteStream(destPath));
  console.log("Downloaded to:", destPath);
}

async function extract7z(filePath: string, destination: string): Promise<void> {
  console.log("Extracting...");
  await new Promise<void>((resolve, reject) => {
    const extractor = Seven.extractFull(filePath, destination, {
      $bin: undefined, // let node-7z find binary
      recursive: true,
      overwrite: 'a',
    });

    extractor.on("end", () => resolve());
    extractor.on("error", (err: any) => reject(err));
  });

  await fs.rm(filePath);
  console.log("Extraction complete.");
}

async function main() {
  try {
    await fs.access(path.resolve(outputDir, "libmpv-2.dll"));
    return;
  } catch {}
  try {
    console.log("Installing libmpv...");
    const url = await getLatestAssetUrl();
    const fileName = path.basename(url);
    const destPath = path.join(outputDir, fileName);

    await downloadFile(url, destPath);
    await extract7z(destPath, outputDir);
  } catch (err) {
    console.error("Failed:", err);
  }
}

main();
