<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import {settingDeveloperMode, settingUiShowShuffleButton} from "$lib/stores/setting";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import ToastController from "$lib/controllers/ToastController";

function onDeveloperModeChange(e: Event & {
    currentTarget: EventTarget & HTMLInputElement;
}) {
    settingDeveloperMode.set(e.currentTarget.checked);
    PersistentStoreController.developerMode.set(e.currentTarget.checked);
    ToastController.info(`Developer mode is ${e.currentTarget.checked ? "enabled" : "disabled"}`);
}
</script>

<SettingLabel
    title="Developer"
    description="Logging and debugging purposes."/>

<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="checkbox"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                checked={$settingDeveloperMode}
                onchange={onDeveloperModeChange}
        />
        <div>Developer Mode</div>
    </label>
</SettingInput>