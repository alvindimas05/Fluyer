<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import { SidebarType } from "$lib/home/sidebar/types.js";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import MenuItem from "$lib/home/menu/MenuItem.svelte";
import UIController from "$lib/controllers/UIController";
import { isDesktop } from "$lib/platform";
</script>

<Sidebar type={SidebarType.Left}>
    <p class="text-[1.2rem] md:text-[1.5rem] font-semibold px-3 py-2">Menu</p>
    <MenuItem label="Settings" icon={IconType.Settings}
              onclick={() => PageController.goto(PageRoutes.SETTINGS)}/>
    <MenuItem label="Full Play Screen" icon={IconType.Fullscreen}
              onclick={() => PageController.goto(PageRoutes.PLAY)}/>
    {#if isDesktop()}
        <MenuItem label="Equalizer" icon={IconType.Equalizer}
                  onclick={() => UIController.toggleEqualizer(true)}/>
    {/if}
    <MenuItem label="Visualizer" icon={IconType.Equalizer}
              onclick={() => PageController.goto(PageRoutes.VISUALIZER)}/>
</Sidebar>