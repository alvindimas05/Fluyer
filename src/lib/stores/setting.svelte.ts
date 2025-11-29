import { SettingAnimatedBackgroundType } from "$lib/settings/animated-background/types";

const settingStore = $state({
    animatedBackground: {
        trigger: "",
        type: SettingAnimatedBackgroundType.Pallete,
    },

    ui: {
        showRepeatButton: true,
        showShuffleButton: true,
        play: {
            showBackButton: true,
            showVolume: true,
        },
    },

    developerMode: false,
    bitPerfectMode: false,
});

export default settingStore;
