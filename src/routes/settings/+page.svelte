<script lang="ts">
import Icon from "$lib/ui/icon/Icon.svelte";
import { IconType } from "$lib/ui/icon/types";
import SettingMusicPaths from "$lib/features/settings/music_paths/SettingMusicPaths.svelte";
import { isMobile } from "$lib/platform";
import SettingAnimatedBackground from "$lib/features/settings/animated_background/SettingAnimatedBackground.svelte";
import SettingUserInteface from "$lib/features/settings/user_interface/SettingUserInterface.svelte";
import SettingDeveloper from "$lib/features/settings/developer/SettingDeveloper.svelte";
import SettingIconTheme from "$lib/features/settings/icon_theme/SettingIconTheme.svelte";
import SettingMusicPlayerConfiguration from "$lib/features/settings/music_player_configuration/SettingMusicPlayerConfiguration.svelte";
import View from "$lib/ui/components/View.svelte";
import Button from "$lib/ui/components/Button.svelte";
import mobileStore from "$lib/stores/mobile.svelte";
import PageService from "$lib/services/PageService.svelte";
</script>

<div class="w-full h-full md:px-3 md:pb-4 md:pt-10">
    <View class="w-full h-full rounded-xl">
        <div class="w-full h-full grid grid-rows-[min-content_auto_min-content]"
            style="padding-top: {isMobile() ? mobileStore.statusBarHeight : 20}px;
                padding-bottom: {mobileStore.navigationBarHeight > 0 ? mobileStore.navigationBarHeight : 24}px;">
            <p class="text-2xl font-semibold mb-4 mx-5">Settings</p>
            <div class="w-full overflow-auto mb-3 scrollbar-hidden">
                <SettingMusicPlayerConfiguration />
                <SettingMusicPaths />
                <SettingAnimatedBackground />
                <SettingIconTheme />
                <SettingUserInteface />
                <SettingDeveloper />
            </div>
            <Button class="w-fit mx-5 px-3 py-2 rounded
                grid grid-cols-[min-content_auto] items-center gap-2"
                onclick={() => PageService.back()}
                glassShineSize="sm">
                <div class="w-4"><Icon type={IconType.Back}/></div>
                <div>Back</div>
            </Button>
        </div>
    </View>
</div>
