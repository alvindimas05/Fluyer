<script lang="ts">
	import { DotLottieSvelte } from '@lottiefiles/dotlottie-svelte';
	import Button from '$lib/ui/components/Button.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
</script>

<div
	class="fixed left-0 top-0 z-10 h-full w-full bg-gray-700/80 text-center font-semibold"
	style="padding-top: {sidebarStore.swipeMinimumTop}px;"
>
	<div class="h-28 overflow-visible">
		<div class="h-40">
			<DotLottieSvelte loop autoplay src="/lotties/swipe-left.json" />
		</div>
	</div>
	<div>Swipe left to open the playlist</div>

	<div class="h-28 overflow-visible">
		<div class="h-40">
			<DotLottieSvelte loop autoplay src="/lotties/swipe-right.json" />
		</div>
	</div>
	<div>Swipe right to open the menu</div>

	<div class="mt-3">Swipe in the opposite direction to close</div>

	<div class="grid w-full justify-center">
		<Button
			class="mt-3 w-fit rounded-md px-3 py-2"
			onclick={() => (mobileStore.showSwipeGuide = false)}>Got it</Button
		>
	</div>
</div>
