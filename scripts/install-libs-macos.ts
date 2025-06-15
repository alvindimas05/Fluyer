import path from "path";
import { downloadFile, extractZip } from "./install-helpers";
import fs from "fs/promises";
import axios from "axios";

const confPath = path.resolve("src-tauri", "tauri.macos.conf.json");
const destPath = path.resolve("src-tauri", "libs");
const arch = process.arch === 'arm64' ? 'arm64' : 'x86_64';
const zipName = `macos-libs-${arch}.zip`;
const zipPath = path.resolve(destPath, zipName);

const filelistURL = `https://raw.githubusercontent.com/alvindimas05/Fluyer-API/refs/heads/main/filelist-${arch}.txt`;

async function updateConfWithFileList() {
	const res = await axios.get(filelistURL);
	const lines = res.data.split("\n").filter((line: string) => line.trim() !== "");
	const libPaths = lines.map((file: string) => `./libs/${file}`);

	const json = { bundle: { macOS: { frameworks: libPaths } } };

	await fs.writeFile(confPath, JSON.stringify(json, null, 4));
	console.log(`Updated conf with dylib paths at ${confPath}.`);
}

async function main() {
	// Assume it only contains .gitignore
	if ((await fs.readdir(destPath)).length > 1) return;

	console.log("Installing MacOS libs...");
	await downloadFile(
		`https://github.com/alvindimas05/Fluyer-API/releases/latest/download/${zipName}`,
		zipPath,
	);
	await extractZip(zipPath, destPath);
	await fs.symlink(
		path.resolve(destPath, "libmpv.2.dylib"),
		path.resolve(destPath, "libmpv.dylib"),
	);

	await updateConfWithFileList();
}

main();
