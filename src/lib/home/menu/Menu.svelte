<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import { SidebarType } from "$lib/home/sidebar/types.js";
import PageController from "$lib/controllers/PageController";
import { PageRoutes } from "$lib/pages";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import MenuItem from "$lib/home/menu/MenuItem.svelte";
</script>

<Sidebar type={SidebarType.Left}>
    <p class="text-[1.2rem] md:text-[1.5rem] font-semibold p-3">Menu</p>
    <MenuItem label="Settings" icon={IconType.Settings}
              onclick={() => PageController.goto(PageRoutes.SETTINGS)}/>
    <MenuItem label="Full Play Screen" icon={IconType.Fullscreen}
              onclick={() => PageController.goto(PageRoutes.PLAY)}/>
</Sidebar>