import path from 'path';
import fs from 'fs/promises';
import { installBass } from './install-bass';
import { installFFmpeg } from './install-ffmpeg';

async function init() {
	try {
		await fs.access(path.resolve('.env'));
	} catch {
		await fs.copyFile(path.resolve('.env.example'), path.resolve('.env'));
		console.log(
			'Missing .env! The file is now copied from .env.example and make sure to configure it before running.'
		);
	}
	const promises = [installBass(), installFFmpeg()];
	await Promise.all(promises);
}

init().catch(console.error);
