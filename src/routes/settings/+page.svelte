<script lang="ts">
import PageController from "$lib/controllers/PageController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import SettingMusicPaths from "$lib/settings/music-paths/SettingMusicPaths.svelte";
import { isMobile } from "$lib/platform";
import SettingAnimatedBackground from "$lib/settings/animated-background/SettingAnimatedBackground.svelte";
import {
	mobileNavigationBarHeight,
	mobileStatusBarHeight,
} from "$lib/stores/mobile";
import { settingIsLoading } from "$lib/stores/setting";
import SettingUserInteface from "$lib/settings/user-interface/SettingUserInteface.svelte";
import SettingDeveloper from "$lib/settings/developer/SettingDeveloper.svelte";
import SettingIconTheme from "$lib/settings/icon-theme/SettingIconTheme.svelte";
import Glass from "$lib/glass/Glass.svelte";
import SettingMusicPlayerConfiguration
    from "$lib/settings/music-player-configuration/SettingMusicPlayerConfiguration.svelte";
import View from "$lib/components/View.svelte";
let isLoading = $derived($settingIsLoading);
</script>

<div class="w-full h-full md:px-3 md:pb-4 md:pt-10">
    <View class="w-full h-full rounded-xl">
        <div class="w-full h-full grid grid-rows-[min-content_auto_min-content] py-5 text-white"
            style="{isMobile() && `padding-top: ${$mobileStatusBarHeight}px;`}
                padding-bottom: {$mobileNavigationBarHeight > 0 ? $mobileNavigationBarHeight : 24}px;">
            <p class="text-2xl font-semibold text-white mb-4 mx-5">Settings</p>
            <div class="w-full overflow-auto mb-3 scrollbar-hidden">
                <SettingMusicPlayerConfiguration />
                <SettingMusicPaths />
                <SettingAnimatedBackground />
                <SettingIconTheme />
                <SettingUserInteface />
                <SettingDeveloper />
            </div>
            <button class="w-fit bg-white/5 border border-white/30 rounded-lg shadow-md mx-5 px-3 py-2
                grid grid-cols-[min-content_auto] items-center gap-2
                hover:bg-white/10 hover:border-white/45"
                onclick={() => PageController.back()}>
                <div class="w-4"><Icon type={IconType.Back}/></div>
                <div>Back</div>
            </button>
        </div>
    </View>
</div>
