import path from "path";
import { downloadFile, extractZip } from "./install-helpers";
import * as fs from "fs/promises";
import * as os from "os";
import * as crypto from "crypto";

const VERSION = 24;
const destPath = path.resolve("src-tauri", "libs");
const libs = ["bass", "bassmix", "bassflac"];

async function installLib(name: string) {
    const downloadName = crypto.randomUUID();
    const downloadPath = path.join(destPath, `${downloadName}.zip`);
    const extractPath = path.join(destPath, downloadName);
    const libWindowsPath = path.join(extractPath, "c", "x64", `${name}.lib`);

    let platform = "";

    let libPath = "";
    switch (os.platform()) {
        case "win32":
            platform = "";
            libPath = path.join(extractPath, "x64", `${name}.dll`);
            break;
        case "darwin":
            platform = "-osx";
            libPath = path.join(extractPath, `lib${name}.dylib`);
            break;
        case "linux":
            platform = "-linux";
            let archFolder = "";
            switch (os.arch()) {
                case "x64":
                    archFolder = "x86_64";
                    break;
                case "ia32":
                    archFolder = "x86";
                    break;
                case "arm":
                    archFolder = "armhf";
                    break;
                case "arm64":
                    archFolder = "aarch64";
                    break;
                default:
                    throw new Error(`Unsupported architecture: ${os.arch()}`);
            }
            libPath = path.join(extractPath, "libs", archFolder, `lib${name}.so`);
            break;
    }

    const destLibPath = path.join(destPath, path.basename(libPath));

    try {
        await fs.access(destLibPath);
        console.log(`${name} is already installed. Reinstalling...`);
        await fs.rm(destLibPath);
    } catch (e) { }

    console.log(`Installing ${name}...`);

    await downloadFile(`https://www.un4seen.com/files/${name}${VERSION}${platform}.zip`, downloadPath);
    await extractZip(downloadPath, extractPath);

    await fs.copyFile(libPath, destLibPath);

    if (os.platform() === "win32") {
        const destLibWindowsPath = path.join(destPath, path.basename(libWindowsPath));
        await fs.copyFile(libWindowsPath, destLibWindowsPath);
    }

    try {
        await fs.rm(downloadPath);
    } catch (e) { }
    try {
        await fs.rm(extractPath, { recursive: true, force: true });
    } catch (e) { }
}

async function main() {
    await fs.mkdir(destPath, { recursive: true });

    const promises: Promise<void>[] = [];
    for (const lib of libs) {
        promises.push(installLib(lib));
    }
    await Promise.all(promises);
}

main();