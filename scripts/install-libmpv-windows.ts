import axios from "axios";
import * as path from "path";
import fs from "fs/promises";
import { downloadFile, extract7z } from "./install-helpers";

const owner = "shinchiro";
const repo = "mpv-winbuild-cmake";
const outputDir = path.resolve("src-tauri", "libs");
const libName = "libmpv-2.dll";
const libPath = path.resolve(outputDir, "..", libName);

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

async function main() {
  try {
    await fs.access(libPath);
    return;
  } catch {}
  try {
    console.log("Installing libmpv...");
    const url = await getLatestAssetUrl();
    const fileName = path.basename(url);
    const destPath = path.join(outputDir, fileName);

    await downloadFile(url, destPath);
    await extract7z(destPath, outputDir);
    await fs.rename(path.resolve(outputDir, libName), libPath);
    console.log(`Moving ${libName} to`, libPath);
  } catch (err) {
    console.error("Failed:", err);
  }
}

main();
