<script lang="ts">
import PageController from "$lib/controllers/PageController";
import Icon from "$lib/icon/Icon.svelte";
import {IconType} from "$lib/icon/types";
import SettingMusicPaths from "$lib/settings/music-paths/SettingMusicPaths.svelte";
import {isDesktop} from "$lib/platform";
import SettingAnimatedBackground from "$lib/settings/animated-background/SettingAnimatedBackground.svelte";
let isLoading = $state(false);
</script>

<div class="w-full h-full px-4 pb-4 pt-8">
    <div class="w-full h-full p-6 bg-gray-700 bg-opacity-30 rounded-xl shadow-xl">
        <div class="w-full h-full grid grid-rows-[min-content_auto_min-content] rounded-lg border border-white/20 p-4 bg-white/5">
            <p class="text-2xl font-semibold text-white mb-4">Settings</p>
            <div class="overflow-auto mb-3 scrollbar-hidden">
                {#if isDesktop()}
                    <SettingMusicPaths />
                {/if}
                <SettingAnimatedBackground />
            </div>
            <div>
                <button
                    class="rounded text-white text-start font-medium tracking-wide px-4 py-2
                        bg-gradient-to-r from-white/10 to-white/5 hover:from-white/20 hover:to-white/10
                        focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200 shadow-md
                        hover:shadow-lg backdrop-blur-md grid grid-cols-[min-content_auto] items-center gap-2"
                    onclick={() => PageController.back()}
                    disabled={isLoading}
                >
                    <div class="w-4"><Icon type={IconType.Back}/></div>
                    <div>Back</div>
                </button>
            </div>
        </div>
    </div>
</div>
