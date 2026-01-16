<script lang="ts">
	import SettingLabel from '$lib/features/settings/SettingLabel.svelte';
	import SettingInput from '$lib/features/settings/SettingInput.svelte';
	import SettingButton from '$lib/features/settings/SettingButton.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import { invoke } from '@tauri-apps/api/core';
	import { CommandRoutes } from '$lib/constants/CommandRoutes';
	import { isDesktop } from '$lib/platform';
	import settingStore from '$lib/stores/setting.svelte.js';
	import PersistentStoreService from '$lib/services/PersistentStoreService.svelte.js';
	import ToastService from '$lib/services/ToastService.svelte.js';

	function onDeveloperModeChange(
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) {
		settingStore.developerMode = e.currentTarget.checked;
		PersistentStoreService.developerMode.set(e.currentTarget.checked);
		ToastService.info(`Developer mode is ${e.currentTarget.checked ? 'enabled' : 'disabled'}`);
	}

	async function saveLog() {
		await invoke(CommandRoutes.DEVELOPER_SAVE_LOG);
	}
</script>

<SettingLabel title="Developer" description="Logging and debugging purposes." />

<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="checkbox"
			class="h-4 w-4 rounded border-white/40 bg-transparent accent-white transition focus:ring-2 focus:ring-white/30"
			checked={settingStore.developerMode}
			onchange={onDeveloperModeChange}
		/>
		<div>Developer Mode</div>
	</label>
</SettingInput>
{#if settingStore.developerMode && isDesktop()}
	<SettingButton label="Save Log" icon={IconType.SaveLog} onclick={saveLog} />
{/if}
