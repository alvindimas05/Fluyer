import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from '@tauri-apps/api/app';
import ToastService from './ToastService.svelte';
import { isMacos, isWindows, isLinux } from '$lib/platform';
import sleep from 'sleep-promise';
import { openUrl } from '@tauri-apps/plugin-opener';
import PersistentStoreService from './PersistentStoreService.svelte';
import TauriUpdateAPI from '$lib/tauri/TauriUpdateAPI';


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
				const currentVersion = await getVersion();
				const latestVersion = await TauriUpdateAPI.checkUpdate(currentVersion);
				if (!latestVersion) return;

				const skippedVersion = await PersistentStoreService.skippedUpdateVersion.get();
				if (skippedVersion === latestVersion) return;

				const downloadUrl = import.meta.env.VITE_UPDATE_URL;

				await sleep(3000);
				ToastService.info(`Would you like to update to version ${latestVersion}?`, 0, [
					{
						label: 'Get Latest Version',
						onClick: () => {
							openUrl(downloadUrl);
						}
					},
					{
						label: 'Skip This Version',
						onClick: () => {
							PersistentStoreService.skippedUpdateVersion.set(latestVersion);
						}
					},
					{
						label: 'Remind Me Later',
						onClick: () => {}
					}
				]);
			}
		} catch (error) {
			console.error('Update check fail:', error);
		}
	}
}

const UpdateService = new UpdateServiceImpl();
export default UpdateService;
