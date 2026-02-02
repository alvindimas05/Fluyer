<script lang="ts">
	import Icon from '$lib/ui/icon/Icon.svelte';
	import { IconType } from '$lib/ui/icon/types';
	import SettingMusicPaths from '$lib/features/settings/music_paths/SettingMusicPaths.svelte';
	import { isMobile } from '$lib/platform';
	import SettingAnimatedBackground from '$lib/features/settings/animated_background/SettingAnimatedBackground.svelte';
	import SettingUserInteface from '$lib/features/settings/user_interface/SettingUserInterface.svelte';
	import SettingDeveloper from '$lib/features/settings/developer/SettingDeveloper.svelte';
	import SettingIconTheme from '$lib/features/settings/icon_theme/SettingIconTheme.svelte';
	import SettingMusicPlayerConfiguration from '$lib/features/settings/music_player_configuration/SettingMusicPlayerConfiguration.svelte';
	import View from '$lib/ui/components/View.svelte';
	import Button from '$lib/ui/components/Button.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import PageService from '$lib/services/PageService.svelte';
	import { onMount } from 'svelte';

	let disableBorder = $state(false);

	function onResize() {
		disableBorder = window.innerWidth < 768;
	}
	onMount(onResize);
</script>

<svelte:window onresize={onResize} />
<div class="h-full w-full md:px-3 md:pb-4 md:pt-10">
	<View class="h-full w-full rounded-xl {disableBorder && '!border-0 !shadow-none'}">
		<div
			class="grid h-full w-full grid-rows-[min-content_auto_min-content]"
			style="padding-top: {isMobile() ? mobileStore.statusBarHeight : 20}px;
                padding-bottom: {mobileStore.navigationBarHeight > 0
				? mobileStore.navigationBarHeight
				: 24}px;"
		>
			<p class="mx-5 mb-4 text-2xl font-semibold">Settings</p>
			<div class="scrollbar-hidden mb-3 w-full overflow-auto">
				<SettingMusicPlayerConfiguration />
				<SettingMusicPaths />
				<SettingAnimatedBackground />
				<SettingIconTheme />
				<SettingUserInteface />
				<SettingDeveloper />
			</div>
			<Button
				class="mx-5 grid w-fit grid-cols-[min-content_auto] items-center
                gap-2 rounded px-3 py-2"
				onclick={() => PageService.back()}
				glassShineSize="sm"
			>
				<div class="w-4"><Icon type={IconType.Back} /></div>
				<div>Back</div>
			</Button>
		</div>
	</View>
</div>
