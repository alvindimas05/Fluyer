import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from '@tauri-apps/api/app';
import ToastService from './ToastService.svelte';
import { isMacos, isWindows, isLinux } from '$lib/platform';
import sleep from 'sleep-promise';
import { openUrl } from '@tauri-apps/plugin-opener';
import PersistentStoreService from './PersistentStoreService.svelte';

function parseVersion(v: string): number[] {
	return v.replace(/^v/, '').split('.').map(Number);
}

function isNewerVersion(current: string, latest: string): boolean {
	const curr = parseVersion(current);
	const late = parseVersion(latest);
	for (let i = 0; i < Math.max(curr.length, late.length); i++) {
		const c = curr[i] || 0;
		const l = late[i] || 0;
		if (l > c) return true;
		if (c > l) return false;
	}
	return false;
}

class UpdateServiceImpl {
	async checkForUpdates() {
		try {
			if (isWindows() || isMacos()) {
				const update = await check();
				if (!update) return;

				const skippedVersion = await PersistentStoreService.skippedUpdateVersion.get();
				if (skippedVersion === update.version) return;

				await sleep(3000);
				ToastService.info(`Would you like to update to version ${update.version}?`, 0, [
					{
						label: 'Update & Restart',
						onClick: async () => {
							ToastService.info('Downloading update...', 0);
							await update.downloadAndInstall();
							ToastService.info('Update installed. Restarting...', 2000);
							await relaunch();
						}
					},
					{
						label: 'Skip This Version',
						onClick: () => {
							PersistentStoreService.skippedUpdateVersion.set(update.version);
						}
					},
					{
						label: 'Remind Me Later',
						onClick: () => {}
					}
				]);
			} else {
				const res = await fetch(
					'https://github.com/alvindimas05/Fluyer/releases/latest/download/latest.json'
				);
				if (!res.ok) return;
				const data = await res.json();
				if (!data || !data.version) return;

				const skippedVersion = await PersistentStoreService.skippedUpdateVersion.get();
				if (skippedVersion === data.version) return;

				const currentVersion = await getVersion();
				if (isNewerVersion(currentVersion, data.version)) {
					const downloadUrl = import.meta.env.VITE_UPDATE_URL;

					await sleep(3000);
					ToastService.info(`Would you like to update to version ${data.version}?`, 0, [
						{
							label: 'Get Latest Version',
							onClick: () => {
								openUrl(downloadUrl);
							}
						},
						{
							label: 'Skip This Version',
							onClick: () => {
								PersistentStoreService.skippedUpdateVersion.set(data.version);
							}
						},
						{
							label: 'Remind Me Later',
							onClick: () => {}
						}
					]);
				}
			}
		} catch (error) {
			console.error('Update check fail:', error);
		}
	}
}

const UpdateService = new UpdateServiceImpl();
export default UpdateService;
