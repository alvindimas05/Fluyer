<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import settingStore from "$lib/stores/setting.svelte";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import musicStore from "$lib/stores/music.svelte";
import ToastService from "$lib/services/ToastService.svelte";

function onBitPerfectModeChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingStore.bitPerfectMode = e.currentTarget.checked;
	PersistentStoreService.bitPerfectMode.set(e.currentTarget.checked);
	// MusicPlayerService.resetEqualizer();
	musicStore.volume = 1;
	ToastService.info(`Bit Perfect mode is ${e.currentTarget.checked ? "enabled" : "disabled"}`);
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
                checked={settingStore.bitPerfectMode}
                onchange={onBitPerfectModeChange}
        />
        <div>Bit Perfect Mode</div>
    </label>
</SettingInput>