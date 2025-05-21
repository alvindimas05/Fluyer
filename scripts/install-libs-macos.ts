import path from "path";
import { downloadFile, extractZip } from "./install-helpers";
import fs from "fs/promises";

const destPath = path.resolve("src-tauri", "libs");
const zipPath = path.resolve(destPath, "macos-libs.zip");
async function main() {
	// Assume it only contains .gitignore
	if ((await fs.readdir(destPath)).length > 1) return;

	console.log("Installing MacOS libs...");
	await downloadFile(
		"https://github.com/alvindimas05/Fluyer-API/releases/latest/download/macos-libs.zip",
		zipPath,
	);
	await extractZip(zipPath, destPath);
	await fs.symlink(
		path.resolve(destPath, "libmpv.2.dylib"),
		path.resolve(destPath, "libmpv.dylib"),
	);
}

main();
