import * as path from "path";
import fs from "fs/promises";
import { downloadFile, extractZip } from "./install-helpers";

// Parse CLI arguments
const VALID_ARCHS = ["arm64-v8a", "armeabi-v7a", "x86", "x86_64"] as const;
type Arch = typeof VALID_ARCHS[number];

// Map Tauri/Rust arch names to Android NDK names
const ARCH_ALIASES: Record<string, Arch> = {
    "aarch64": "arm64-v8a",
    "armv7": "armeabi-v7a",
    "i686": "x86",
    // Direct mappings
    "arm64-v8a": "arm64-v8a",
    "armeabi-v7a": "armeabi-v7a",
    "x86": "x86",
    "x86_64": "x86_64",
};

function parseArgs(): Arch {
    const args = process.argv.slice(2);

    if (args.includes("--help") || args.includes("-h")) {
        console.log(`Usage: bun android:init [--arch <architecture>]

Options:
  --arch, -a    Target architecture (default: arm64-v8a)
                Valid values: ${VALID_ARCHS.join(", ")}
                Aliases: aarch64 -> arm64-v8a, armv7 -> armeabi-v7a, i686 -> x86
  --help, -h    Show this help message

Examples:
  bun android:init
  bun android:init --arch arm64-v8a
  bun android:init -a aarch64
  bun android:init -a x86_64
`);
        process.exit(0);
    }

    const archIndex = args.findIndex(arg => arg === "--arch" || arg === "-a");
    if (archIndex !== -1 && args[archIndex + 1]) {
        const inputArch = args[archIndex + 1];
        const arch = ARCH_ALIASES[inputArch];
        if (!arch) {
            console.error(`Invalid architecture: ${inputArch}`);
            console.error(`Valid architectures: ${VALID_ARCHS.join(", ")}`);
            console.error(`Aliases: aarch64, armv7, i686`);
            process.exit(1);
        }
        return arch;
    }

    return "arm64-v8a"; // default
}

const ARCH = parseArgs();
console.log(`Target architecture: ${ARCH}\n`);

const version = "6.0";
const filename = "ffmpeg-kit-main.aar";
const destDir = path.resolve(
    "src-tauri",
    "tauri-plugin-fluyer",
    "android",
    "libs",
);
const ffmpegDestPath = path.resolve(destDir, filename);
const downloadUrl = `https://github.com/alvindimas05/ffmpeg-kit/releases/download/v${version}/${filename}`;

const VERSION = 24;
const bassDestPath = path.resolve("src-tauri", "gen", "android", "app", "src", "main", "jniLibs");
const libs = ["bass", "bassmix", "bassflac"];

async function installLib(name: string) {
    const downloadName = crypto.randomUUID();
    const downloadPath = path.join(bassDestPath, `${downloadName}.zip`);
    const extractPath = path.join(bassDestPath, downloadName);

    const libPath = path.join(extractPath, "libs", ARCH, `lib${name}.so`);
    const destLibPath = path.join(bassDestPath, ARCH, path.basename(libPath));

    try {
        await fs.access(destLibPath);
        console.log(`${name} is already installed. Reinstalling...`);
        await fs.rm(destLibPath);
    } catch (e) { }

    console.log(`Installing ${name}...`);

    await downloadFile(`https://www.un4seen.com/files/${name}${VERSION}-android.zip`, downloadPath);
    await extractZip(downloadPath, extractPath);

    await fs.mkdir(path.join(bassDestPath, ARCH), { recursive: true });
    await fs.copyFile(libPath, destLibPath);

    try {
        await fs.rm(downloadPath);
    } catch (e) { }
    try {
        await fs.rm(extractPath, { recursive: true, force: true });
    } catch (e) { }
}

async function installBass() {
    await fs.mkdir(bassDestPath, { recursive: true });

    const promises: Promise<void>[] = [];
    for (const lib of libs) {
        promises.push(installLib(lib));
    }
    await Promise.all(promises);
}

async function installFFmpeg() {
    try {
        await fs.access(ffmpegDestPath);
        console.log("FFmpeg for Android is already installed. Reinstalling...");
        await fs.rm(ffmpegDestPath);
    } catch { }
    try {
        console.log("Installing FFmpeg for Android...");
        await fs.mkdir(destDir, { recursive: true });
        await downloadFile(downloadUrl, ffmpegDestPath);
    } catch (err) {
        console.error("Failed:", err);
    }
}

Promise.all([installBass(), installFFmpeg()]);