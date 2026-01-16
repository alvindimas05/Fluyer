<script lang="ts">
	import SettingLabel from '$lib/features/settings/SettingLabel.svelte';
	import SettingInput from '$lib/features/settings/SettingInput.svelte';
	import { IconThemeType } from '$lib/ui/icon/types';
	import iconStore from '$lib/stores/icon.svelte.js';
	import PersistentStoreService from '$lib/services/PersistentStoreService.svelte.js';
	import ToastService from '$lib/services/ToastService.svelte.js';

	async function onMethodChange(
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) {
		const value = <IconThemeType>e.currentTarget.value;
		if (value === iconStore.theme) return;
		iconStore.theme = value;
		await PersistentStoreService.iconTheme.set(value);
		ToastService.info('Icon theme changed to ' + value);
	}
</script>

<SettingLabel title="Icon Theme" description="The theme for the icons." />
<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="radio"
			name="iconTheme"
			class="h-4 w-4 rounded border-white/40 bg-transparent accent-white transition focus:ring-2 focus:ring-white/30"
			value={IconThemeType.Phosphor}
			checked={iconStore.theme === IconThemeType.Phosphor}
			onchange={onMethodChange}
		/>
		<div>
			<span class="font-semibold">Phosphor</span> by Phosphor Icons
		</div>
	</label>
</SettingInput>
<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="radio"
			name="iconTheme"
			class="h-4 w-4 rounded border-white/40 bg-transparent accent-white transition focus:ring-2 focus:ring-white/30"
			value={IconThemeType.Material}
			checked={iconStore.theme === IconThemeType.Material}
			onchange={onMethodChange}
		/>
		<div>
			<span class="font-semibold">Material</span> by Google
		</div>
	</label>
</SettingInput>
<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="radio"
			name="iconTheme"
			class="h-4 w-4 rounded border-white/40 bg-transparent accent-white transition focus:ring-2 focus:ring-white/30"
			value={IconThemeType.Lucide}
			checked={iconStore.theme === IconThemeType.Lucide}
			onchange={onMethodChange}
		/>
		<div>
			<span class="font-semibold">Lucide</span> by Lucide Icons
		</div>
	</label>
</SettingInput>
