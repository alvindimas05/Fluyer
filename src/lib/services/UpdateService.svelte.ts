import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import ToastService from './ToastService.svelte';

class UpdateServiceImpl {
	async checkForUpdates() {
		try {
			const update = await check();
			if (!update) return;
			ToastService.info(
				`Update ${update.version} available.`,
				0,
				[
					{
						label: 'Update',
						onClick: async () => {
							ToastService.info('Downloading update...', 0);
							await update.downloadAndInstall();
							ToastService.info('Update installed. Restarting...', 2000);
							await relaunch();
						}
					},
					{
						label: 'Cancel',
						onClick: () => {} // automatically closes due to Toast component behavior
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
