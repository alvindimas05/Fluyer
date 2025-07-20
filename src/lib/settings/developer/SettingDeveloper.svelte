<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import {settingDeveloperMode, settingUiShowShuffleButton} from "$lib/stores/setting";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import ToastController from "$lib/controllers/ToastController";
import SettingButton from "$lib/settings/SettingButton.svelte";
import {IconType} from "$lib/icon/types";
import {invoke} from "@tauri-apps/api/core";
import {CommandRoutes} from "$lib/commands";

function onDeveloperModeChange(e: Event & {
    currentTarget: EventTarget & HTMLInputElement;
}) {
    settingDeveloperMode.set(e.currentTarget.checked);
    PersistentStoreController.developerMode.set(e.currentTarget.checked);
    ToastController.info(`Developer mode is ${e.currentTarget.checked ? "enabled" : "disabled"}`);
}

async function saveLog(){
    await invoke(CommandRoutes.DEVELOPER_SAVE_LOG);
    ToastController.info("Log saved at Downloads");
}

async function saveMpvLog(){
    await invoke(CommandRoutes.DEVELOPER_SAVE_MPV_LOG);
    ToastController.info("MPV log saved at Downloads");
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
{#if $settingDeveloperMode}
    <SettingButton
            label="Save Log"
            icon={IconType.SaveLog}
            onclick={saveLog}/>
    <SettingButton
            label="Save MPV Log"
            icon={IconType.SaveLog}
            onclick={saveMpvLog} />
{/if}