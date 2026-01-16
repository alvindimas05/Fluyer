import * as path from 'path';
import fs from 'fs/promises';
import os from 'os';
import { downloadFile, extractTarGz } from './install-helpers';
const version = '8.0';
const downloadOutputDir = path.resolve('src-tauri', 'libs');
const outputDir = path.resolve(downloadOutputDir, 'ffmpeg');

function getLibName() {
	let versionName: string;
	switch (os.platform()) {
		case 'win32':
			versionName = 'x86_64-w64-mingw32';
			break;
		case 'darwin':
			versionName = process.arch === 'arm64' ? 'arm64-apple-macos11' : 'x86_64-apple-macos10.9';
			break;
		case 'linux':
			versionName = 'x86_64-linux-gnu';
			break;
		default:
			throw new Error('Unsupported platform');
	}
	return `ffmpeg-${version}-audio-${versionName}`;
}
function getDownloadUrl(): string {
	return `https://github.com/alvindimas05/ffmpeg-build/releases/download/v${version}/${getLibName()}.tar.gz`;
}

async function main() {
	try {
		await fs.access(outputDir);
		console.log('FFmpeg is already installed. Reinstalling...');
		await fs.rm(outputDir, { recursive: true, force: true });
	} catch {}
	try {
		console.log('Installing ffmpeg...');
		const url = getDownloadUrl();
		const fileName = path.basename(url);
		const destPath = path.join(downloadOutputDir, fileName);

		await downloadFile(url, destPath);
		await extractTarGz(destPath, downloadOutputDir);

		await fs.rename(path.resolve(downloadOutputDir, getLibName()), outputDir);
	} catch (err) {
		console.error('Failed:', err);
	}
}

main();
