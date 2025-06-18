<script lang="ts">
import Sidebar from "$lib/home/sidebar/Sidebar.svelte";
import {SidebarType} from "$lib/home/sidebar/types.js";
import PageController from "$lib/controllers/PageController";
import {PageRoutes} from "$lib/pages";
</script>

<Sidebar type={SidebarType.Left}>
    <p class="text-[1.5rem] font-semibold p-3">Menu</p>
    <button
        class="text-white text-start font-medium tracking-wide px-4 py-2 bg-gradient-to-r from-white/10 to-white/5 hover:from-white/20 hover:to-white/10 focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200 shadow-md hover:shadow-lg backdrop-blur-md"
        onclick={() => PageController.goto(PageRoutes.SETTINGS)}
    >
        Settings
    </button>
</Sidebar>