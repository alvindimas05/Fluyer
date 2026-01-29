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

	const RESPONSIVE_RULES = [
		[1280, 2.01, .25],
		[1024, 2.01, .3334],
		[768, 2.01, .5],
		[1536, 1.01, .25],
		[1280, 1.01, .3334],
		[768, 1.01, .5],
		[1536, 1.0, .25],
		[1024, 1.0, .3334],
		[768, 1.0, .5]
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
	class="pointer-events-none fixed top-0 z-[100000] flex h-screen flex-col justify-start items-center px-3 md:right-0 md:items-end"
    style="padding-top: {(isMobile() ? mobileStore.statusBarHeight : 0) + filterStore.bar.height}px;
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
