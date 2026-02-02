<script lang="ts">
	import SettingLabel from '$lib/features/settings/SettingLabel.svelte';
	import SettingInput from '$lib/features/settings/SettingInput.svelte';
	import { SettingAnimatedBackgroundType } from '$lib/features/settings/animated_background/types.js';
	import settingStore from '$lib/stores/setting.svelte.js';
	import PersistentStoreService from '$lib/services/PersistentStoreService.svelte';
	import ToastService from '$lib/services/ToastService.svelte';

	async function onMethodChange(
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) {
		const value = <SettingAnimatedBackgroundType>e.currentTarget.value;
		if (value === settingStore.animatedBackground.type) return;
		await PersistentStoreService.animatedBackgroundType.set(value);
		settingStore.animatedBackground.type = value;
		settingStore.animatedBackground.trigger = new Date().toString();

		ToastService.info('Animated Background method changed to ' + value);
	}
</script>

<SettingLabel
	title="Animated Background"
	description="The methods for generating animated backgrounds."
/>
<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="radio"
			name="animatedBackgroundMethod"
			class="h-4 w-4"
			value={SettingAnimatedBackgroundType.Prominent}
			checked={settingStore.animatedBackground.type === SettingAnimatedBackgroundType.Prominent}
			onchange={onMethodChange}
		/>
		<div>
			<span class="font-semibold">Prominent</span> – Extracts the most dominant colors from the image
			for a bold and cohesive style.
		</div>
	</label>
</SettingInput>
<SettingInput>
	<label class="grid cursor-pointer grid-cols-[min-content_auto] items-center gap-3 px-3 py-2">
		<input
			type="radio"
			name="animatedBackgroundMethod"
			class="h-4 w-4"
			value={SettingAnimatedBackgroundType.Pallete}
			checked={settingStore.animatedBackground.type === SettingAnimatedBackgroundType.Pallete}
			onchange={onMethodChange}
		/>
		<div>
			<span class="font-semibold">Palette</span> – Extracts a wide range of colors from the image for
			a more vibrant look.
		</div>
	</label>
</SettingInput>
