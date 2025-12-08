<script lang="ts">
import SettingLabel from "$lib/features/settings/SettingLabel.svelte";
import SettingInput from "$lib/features/settings/SettingInput.svelte";
import { isDesktop } from "$lib/platform";
import settingStore from "$lib/stores/setting.svelte";
import PersistentStoreService from "$lib/services/PersistentStoreService.svelte";
import ToastService from "$lib/services/ToastService.svelte";

function onUiPlayShowBackButtonChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingStore.ui.play.showBackButton = e.currentTarget.checked;
	PersistentStoreService.userInterface.play.showBackButton.set(
		e.currentTarget.checked,
	);
	ToastService.info(
		`Play Back button ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}

function onUiPlayShowVolumeChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingStore.ui.play.showVolume = e.currentTarget.checked;
	PersistentStoreService.userInterface.play.showVolume.set(
		e.currentTarget.checked,
	);
	ToastService.info(
		`Play Volume ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}

function onUiShowRepeatButtonChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingStore.ui.showRepeatButton = e.currentTarget.checked;
	PersistentStoreService.userInterface.showRepeatButton.set(
		e.currentTarget.checked,
	);
	ToastService.info(
		`Repeat button ${e.currentTarget.checked ? "enabled" : "disabled"}`,
	);
}

function onUiShowShuffleButtonChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	settingStore.ui.showShuffleButton = e.currentTarget.checked;
	PersistentStoreService.userInterface.showShuffleButton.set(
		e.currentTarget.checked,
	);
	ToastService.info(
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
                checked={settingStore.ui.showRepeatButton}
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
                checked={settingStore.ui.showShuffleButton}
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
                    checked={settingStore.ui.play.showBackButton}
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
                    checked={settingStore.ui.play.showVolume}
                    onchange={onUiPlayShowVolumeChange}
            />
            <div>Show Volume On Full Play Screen</div>
        </label>
    </SettingInput>
{/if}