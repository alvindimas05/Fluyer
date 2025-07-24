<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import ToastController from "$lib/controllers/ToastController";
import { IconThemeType } from "$lib/icon/types";
import { iconTheme } from "$lib/stores/icon";

async function onMethodChange(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	},
) {
	const value = <IconThemeType>e.currentTarget.value;
	if (value === $iconTheme) return;
	iconTheme.set(value);
	await PersistentStoreController.iconTheme.set(value);
	ToastController.info("Icon theme changed to " + value);
}
</script>

<SettingLabel
    title="Icon Theme"
    description="The theme for the icons."/>
<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="radio"
                name="iconTheme"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                value={IconThemeType.Phosphor}
                checked={$iconTheme === IconThemeType.Phosphor}
                onchange={onMethodChange}
        />
        <div>
            <span class="font-semibold">Phosphor</span> by Phosphor Icons
        </div>
    </label>
</SettingInput>
<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="radio"
                name="iconTheme"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                value={IconThemeType.Material}
                checked={$iconTheme === IconThemeType.Material}
                onchange={onMethodChange}
        />
        <div>
            <span class="font-semibold">Material</span> by Google
        </div>
    </label>
</SettingInput>
<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
                type="radio"
                name="iconTheme"
                class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
                value={IconThemeType.Lucide}
                checked={$iconTheme === IconThemeType.Lucide}
                onchange={onMethodChange}
        />
        <div>
            <span class="font-semibold">Lucide</span> by Lucide Icons
        </div>
    </label>
</SettingInput>