import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import ToastService from './ToastService.svelte';
import { isMacos, isWindows } from '$lib/platform';
import sleep from 'sleep-promise';

class UpdateServiceImpl {
	async checkForUpdates() {
		try {
			if (!(isWindows() || isMacos())) return;

			const update = await check();
			if (!update) return;
			await sleep(3000);
			ToastService.info(
				`Would you like to update to version ${update.version}?`,
				0,
				[
					{
						label: 'Yes',
						onClick: async () => {
							ToastService.info('Downloading update...', 0);
							await update.downloadAndInstall();
							ToastService.info('Update installed. Restarting...', 2000);
							await relaunch();
						}
					},
					{
						label: 'No',
						onClick: () => { }
					}
				]
			);
		} catch (error) {
			console.error('Update check fail:', error);
		}
	}
}

const UpdateService = new UpdateServiceImpl();
export default UpdateService;
