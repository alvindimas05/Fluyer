<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import { SidebarType } from "$lib/home/sidebar/types.js";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import { IconType } from "$lib/icon/types";
import MenuItem from "$lib/home/menu/MenuItem.svelte";
import UIController from "$lib/controllers/UIController";
import { isDesktop } from "$lib/platform";
import { musicListType } from "$lib/stores/music.svelte";
import { MusicListType } from "$lib/home/music/types";
import { settingBitPerfectMode } from "$lib/stores/setting";
</script>

<Sidebar type={SidebarType.Left}>
    <p class="text-[1.2rem] md:text-[1.5rem] font-semibold px-3 py-2">Menu</p>
    <MenuItem label="Play Screen" icon={IconType.Fullscreen}
              onclick={() => PageController.goto(PageRoutes.PLAY)}/>
    <!--{#if isDesktop() && !$settingBitPerfectMode}-->
    <!--    <MenuItem label="Equalizer" icon={IconType.Equalizer}-->
    <!--              onclick={() => UIController.toggleEqualizer(true)}/>-->
    <!--{/if}-->
    <MenuItem label="Visualizer" icon={IconType.Visualizer}
              onclick={() => PageController.goto(PageRoutes.VISUALIZER)}/>
    <MenuItem label="Settings" icon={IconType.Settings}
              onclick={() => PageController.goto(PageRoutes.SETTINGS)}/>
</Sidebar>