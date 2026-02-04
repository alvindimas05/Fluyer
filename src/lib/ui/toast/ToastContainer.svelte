<script lang="ts">
	import { isMobile } from '$lib/platform';
	import ToastService from '$lib/services/ToastService.svelte';
	import filterStore from '$lib/stores/filter.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import { onMount } from 'svelte';
	import Toast from './Toast.svelte';
	import { flip } from 'svelte/animate';
	import { fly } from 'svelte/transition';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import { SidebarType } from '$lib/features/sidebar/types';
	import { page } from '$app/state';
	import { PageRoutes } from '$lib/constants/PageRoutes';

	const RESPONSIVE_RULES = [
		[1280, 2.01, 0.25],
		[1024, 2.01, 0.3334],
		[768, 2.01, 0.5],
		[1536, 1.01, 0.25],
		[1280, 1.01, 0.3334],
		[768, 1.01, 0.5],
		[1536, 1.0, 0.25],
		[1024, 1.0, 0.3334],
		[768, 1.0, 0.5]
	];
	let width = $state(1);
	function updateColumnCount() {
		const w = window.innerWidth;
		const dpi = window.devicePixelRatio;

		for (const [minW, minDppx, cols] of RESPONSIVE_RULES) {
			if (w >= minW && dpi >= minDppx) {
				width = cols;
				return;
			}
		}
		width = 1;
	}

	$effect(() => {
		if (sidebarStore.showType === SidebarType.Right) {
			ToastService.clear();
		}
	});

	onMount(() => {
		updateColumnCount();
	});
</script>

<svelte:window onresize={updateColumnCount} />
<div
	class="pointer-events-none fixed top-0 z-[100000] flex h-screen flex-col items-center justify-start px-3 md:right-0 md:items-end
		{page.url.pathname === PageRoutes.SETTINGS ? 'md:right-5 md:top-10 px-5 md:px-3' : 'md:right-0'}"
	style="padding-top: {(() => {
		let value = isMobile() ? mobileStore.statusBarHeight : 0;
		if ([PageRoutes.HOME, PageRoutes.HOME_PRODUCTION].includes(page.url.pathname)) {
			value += filterStore.bar.height;
		}
		if(page.url.pathname === PageRoutes.SETTINGS && !isMobile()) {
			value += 20;
		}
		return value;
	})()}px;
	width: {width * 100}%;"
>
	{#each ToastService.toasts as toast (toast.id)}
		<div
			animate:flip={{ duration: 300 }}
			in:fly={{ y: -50, duration: 300 }}
			out:fly={{ y: -50, duration: 300 }}
			class="flex w-full justify-center md:justify-end"
		>
			<Toast {toast} />
		</div>
	{/each}
</div>
