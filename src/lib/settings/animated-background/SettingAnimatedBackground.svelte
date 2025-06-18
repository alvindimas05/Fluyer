<script lang="ts">
import SettingLabel from "$lib/settings/SettingLabel.svelte";
import SettingInput from "$lib/settings/SettingInput.svelte";
import {SettingAnimatedBackgroundType} from "$lib/settings/animated-background/types.js";
import {settingAnimatedBackgroundType, settingTriggerAnimatedBackground} from "$lib/stores/setting";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";

async function onMethodChange(e: InputEvent & {
    currentTarget: EventTarget & HTMLInputElement;
}){
    const value = <SettingAnimatedBackgroundType> e.currentTarget.value;
    if(value === $settingAnimatedBackgroundType) return;
    await PersistentStoreController.animatedBackgroundType.set(value);
    await PersistentStoreController.animatedBackgroundType.setStore();
    $settingTriggerAnimatedBackground = (new Date()).toString();
}
</script>

<SettingLabel
    title="Animated Background"
    description="The methods for generating animated backgrounds."/>
<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
            type="radio"
            name="animatedBackgroundMethod"
            class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
            value={SettingAnimatedBackgroundType.Pallete}
            checked={$settingAnimatedBackgroundType === SettingAnimatedBackgroundType.Pallete}
            onchange={onMethodChange}
        />
        <div>
            <span class="font-semibold">Palette</span> – Extracts a wide range of colors from the image for a more vibrant look.
        </div>
    </label>
</SettingInput>

<SettingInput>
    <label class="grid grid-cols-[min-content_auto] items-center gap-3 px-3 py-2 cursor-pointer">
        <input
            type="radio"
            name="animatedBackgroundMethod"
            class="w-4 h-4 accent-white bg-transparent border-white/40 rounded focus:ring-2 focus:ring-white/30 transition"
            value={SettingAnimatedBackgroundType.Prominent}
            checked={$settingAnimatedBackgroundType === SettingAnimatedBackgroundType.Prominent}
            onchange={onMethodChange}
        />
        <div>
            <span class="font-semibold">Prominent</span> – Extracts the most dominant colors from the image for a bold and cohesive style.
        </div>
    </label>
</SettingInput>
