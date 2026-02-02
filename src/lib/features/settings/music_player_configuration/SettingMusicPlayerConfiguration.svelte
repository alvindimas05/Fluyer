<script lang="ts">
	import SettingLabel from '$lib/features/settings/SettingLabel.svelte';
	import SettingInput from '$lib/features/settings/SettingInput.svelte';
	import settingStore from '$lib/stores/setting.svelte.js';
	import PersistentStoreService from '$lib/services/PersistentStoreService.svelte.js';
	import musicStore from '$lib/stores/music.svelte.js';
	import ToastService from '$lib/services/ToastService.svelte.js';

	function onBitPerfectModeChange(
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) {
		settingStore.bitPerfectMode = e.currentTarget.checked;
		PersistentStoreService.bitPerfectMode.set(e.currentTarget.checked);
		// MusicPlayerService.resetEqualizer();
		musicStore.volume = 1;
		ToastService.info(`Bit Perfect mode is ${e.currentTarget.checked ? 'enabled' : 'disabled'}`);
	}
</script>

<SettingLabel
	title="Music Player Configuration"
	description="Adjust how the in-game music player behaves."
/>

<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="checkbox"
			class="h-4 w-4"
			checked={settingStore.bitPerfectMode}
			onchange={onBitPerfectModeChange}
		/>
		<div>Bit Perfect Mode</div>
	</label>
</SettingInput>
