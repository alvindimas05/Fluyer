<script lang="ts">
	import Button from '$lib/ui/components/Button.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';

	let leftLoaded = $state(false);
	let rightLoaded = $state(false);
	let allLoaded = $derived(leftLoaded && rightLoaded);
</script>

<div
	class="fixed left-0 top-0 z-10 h-full w-full bg-gray-700/80 text-center font-semibold transition-opacity duration-300 {allLoaded
		? 'opacity-100'
		: 'pointer-events-none opacity-0'}"
	style="padding-top: {sidebarStore.swipeMinimumTop}px;"
>
	<div class="grid items-center justify-center">
		<video autoplay loop muted playsinline onloadeddata={() => (leftLoaded = true)}>
			<source src="/lotties/swipe-left.webm" type="video/webm" />
		</video>
	</div>
	<div>Swipe left to open the playlist</div>

	<div class="grid items-center justify-center">
		<video autoplay loop muted playsinline onloadeddata={() => (rightLoaded = true)}>
			<source src="/lotties/swipe-right.webm" type="video/webm" />
		</video>
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
