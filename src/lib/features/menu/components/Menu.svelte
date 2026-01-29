<script lang="ts">
	import { PageRoutes } from '$lib/constants/PageRoutes';
	import { IconType } from '$lib/ui/icon/types';
	import Sidebar from '$lib/features/sidebar/components/Sidebar.svelte';
	import MenuItem from '$lib/features/menu/components/MenuItem.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import PageService from '$lib/services/PageService.svelte';
</script>

<Sidebar type={SidebarType.Left}>
	<p class="px-3 py-2 text-[1.2rem] font-semibold md:text-[1.5rem]">Menu</p>
	<MenuItem
		label="Play Screen"
		icon={IconType.Fullscreen}
		onclick={() => PageService.goTo(PageRoutes.PLAY)}
	/>
	<!--{#if isDesktop() && !$settingBitPerfectMode}-->
	<!--    <MenuItem label="Equalizer" icon={IconType.Equalizer}-->
	<!--              onclick={() => UIController.toggleEqualizer(true)}/>-->
	<!--{/if}-->
	<!-- <MenuItem
		label="Visualizer"
		icon={IconType.Visualizer}
		onclick={() => PageService.goTo(PageRoutes.VISUALIZER)}
	/> -->
	<MenuItem
		label="Settings"
		icon={IconType.Settings}
		onclick={() => PageService.goTo(PageRoutes.SETTINGS)}
	/>
</Sidebar>
