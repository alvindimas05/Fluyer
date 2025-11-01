<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import { settingBitPerfectMode } from "$lib/stores/setting";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import ToastController from "$lib/controllers/ToastController";
import MusicController from "$lib/controllers/MusicController";

function onBitPerfectModeChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingBitPerfectMode.set(e.currentTarget.checked);
	PersistentStoreController.bitPerfectMode.set(e.currentTarget.checked);
	MusicController.resetEqualizer();
	MusicController.setVolume(1);
	ToastController.info(
		`Bit Perfect mode is ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}
</script>

<SettingLabel
    title="Music Player Configuration"
    description="Adjust how the in-game music player behaves."/>

<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="checkbox"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                checked={$settingBitPerfectMode}
                onchange={onBitPerfectModeChange}
        />
        <div>Bit Perfect Mode</div>
    </label>
</SettingInput>