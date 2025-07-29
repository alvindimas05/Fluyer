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
let isLoading = $derived($settingIsLoading);
</script>

<div class="w-full h-full md:px-3 md:pb-4 md:pt-10">
        <div class="w-full h-full grid grid-rows-[min-content_auto_min-content]
        p-5 bg-gray-500 bg-opacity-30 rounded-lg shadow-2xl border border-white/20 text-white"
             style="{isMobile() && `padding-top: ${$mobileStatusBarHeight}px;`}
             padding-bottom: {$mobileNavigationBarHeight > 0 ? $mobileNavigationBarHeight : 24}px;">
            <p class="text-2xl font-semibold text-white mb-4">Settings</p>
            <div class="overflow-auto mb-3 scrollbar-hidden">
                <SettingMusicPaths />
                <SettingAnimatedBackground />
                <SettingIconTheme />
                <SettingUserInteface />
                <SettingDeveloper />
            </div>
            <div>
                <button
                    class="rounded text-white text-start font-medium tracking-wide px-4 py-2
                        bg-gradient-to-r from-white/10 to-white/5 hover:from-white/20 hover:to-white/10
                        focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200 shadow-md
                        hover:shadow-lg grid grid-cols-[min-content_auto] items-center gap-2"
                    onclick={() => PageController.back()}
                    disabled={isLoading}
                >
                    <div class="w-4"><Icon type={IconType.Back}/></div>
                    <div>Back</div>
                </button>
            </div>
        </div>
</div>
