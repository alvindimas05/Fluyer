<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import {
	settingUiPlayShowBackButton,
	settingUiPlayShowVolume,
	settingUiShowRepeatButton,
	settingUiShowShuffleButton,
} from "$lib/stores/setting";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import { isDesktop } from "$lib/platform";
import ToastController from "$lib/controllers/ToastController";

function onUiPlayShowBackButtonChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingUiPlayShowBackButton.set(e.currentTarget.checked);
	PersistentStoreController.userInterface.play.showBackButton.set(
		e.currentTarget.checked,
	);
	ToastController.info(
		`Play Back button ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}

function onUiPlayShowVolumeChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingUiPlayShowVolume.set(e.currentTarget.checked);
	PersistentStoreController.userInterface.play.showVolume.set(
		e.currentTarget.checked,
	);
	ToastController.info(
		`Play Volume ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}

function onUiShowRepeatButtonChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingUiShowRepeatButton.set(e.currentTarget.checked);
	PersistentStoreController.userInterface.showRepeatButton.set(
		e.currentTarget.checked,
	);
	ToastController.info(
		`Repeat button ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}

function onUiShowShuffleButtonChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingUiShowShuffleButton.set(e.currentTarget.checked);
	PersistentStoreController.userInterface.showShuffleButton.set(
		e.currentTarget.checked,
	);
	ToastController.info(
		`Shuffle button ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}
</script>

<SettingLabel
    title="User Interface"
    description="Configure how you want the user interface to look like."/>

<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="checkbox"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                checked={$settingUiShowRepeatButton}
                onchange={onUiShowRepeatButtonChange}
        />
        <div>Show Repeat Button</div>
    </label>
</SettingInput>
<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="checkbox"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                checked={$settingUiShowShuffleButton}
                onchange={onUiShowShuffleButtonChange}
        />
        <div>Show Shuffle Button</div>
    </label>
</SettingInput>

{#if isDesktop()}
    <SettingInput>
        <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
            <input
                    type="checkbox"
                    class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                    checked={$settingUiPlayShowBackButton}
                    onchange={onUiPlayShowBackButtonChange}
            />
            <div>Show Back Button On Full Play Screen (You can still press Esc to go back)</div>
        </label>
    </SettingInput>
    <SettingInput>
        <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
            <input
                    type="checkbox"
                    class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                    checked={$settingUiPlayShowVolume}
                    onchange={onUiPlayShowVolumeChange}
            />
            <div>Show Volume On Full Play Screen</div>
        </label>
    </SettingInput>
{/if}