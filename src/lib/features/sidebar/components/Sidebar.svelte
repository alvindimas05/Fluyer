<script lang="ts">
	import { SidebarType } from '$lib/features/sidebar/types';

	interface Props {
		children?: import('svelte').Snippet;
		type: SidebarType;
		class?: string;
	}

	const props = $props();
	let { children, type }: Props = props;

	import { isLinux, isMobile, isWindows } from '$lib/platform';
	import { swipeable } from '@react2svelte/swipeable';
	import type { SwipeEventData } from '@react2svelte/swipeable';
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import View from '$lib/ui/components/View.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import filterStore from '$lib/stores/filter.svelte';

	import { MusicListType } from '$lib/features/music/types';
	import musicStore from '$lib/stores/music.svelte';

	const rules = [
		// xhdpi (DPR > 2.0)
		[1536, 2.01, 0.125], // 2xl → 12.5%
		[1280, 2.01, 0.16667], // xl-xhdpi → 16.6667%
		[1024, 2.01, 0.2], // lg-xhdpi → 20%
		[768, 2.01, 0.25], // md-xhdpi → 25%
		[640, 2.01, 0.33334], // sm-xhdpi → 33.3334%

		// hdpi (1.01 ≤ DPR ≤ 2.0)
		[1536, 1.01, 0.125], // 2xl → 12.5%
		[1280, 1.01, 0.16667], // xl-hdpi → 16.6667%
		[1024, 1.01, 0.2], // lg-hdpi → 20%
		[768, 1.01, 0.25], // md-hdpi → 25%
		[640, 1.01, 0.33334], // sm-hdpi → 33.3334%

		// default (DPR <= 1.0)
		[1536, 0, 0.125], // 2xl → 12.5%
		[1280, 0, 0.16667], // xl → 16.6667%
		[1024, 0, 0.2], // lg → 20%
		[768, 0, 0.25], // md → 25%
		[640, 0, 0.33334] // sm → 33.3334%
	];

	const SWIPE_RANGE = 125;

	let sidebarWidth = $derived(sidebarStore.width);
	let paddingTop = $derived(
		(isMobile() ? mobileStore.statusBarHeight : 0) + filterStore.bar.height
	);

	let isMouseInsideArea = $state(false);
	let isShowing = $state(false);
	let isMounted = $state(false);

	async function onMouseMove(e: MouseEvent) {
		const win = getCurrentWindow();
		const isMaximized = await win.isMaximized();

		const onRightEdge =
			type === SidebarType.Right &&
			e.clientX > window.innerWidth - (isWindows() || !isMaximized ? 12 : 4);

		const onLeftEdge = type === SidebarType.Left && e.clientX < (isMaximized ? 4 : 12);

		const withinVerticalBounds = e.clientY <= window.innerHeight - 8 * 16;

		if ((onRightEdge || onLeftEdge) && withinVerticalBounds && !isMouseInsideArea) {
			isMouseInsideArea = true;
			isShowing = true;
			sidebarStore.showType = type;
			console.log('Showing sidebar:', type);
		}
	}

	async function onMouseLeave(e: MouseEvent) {
		const nearScreenEdge = e.clientX <= 20 || e.clientX >= window.innerWidth - 20;

		if (!isMouseInsideArea || nearScreenEdge) return;

		isShowing = false;
		isMouseInsideArea = false;
		sidebarStore.showType = null;
	}

	function onSwipe(e: CustomEvent<SwipeEventData>) {
		const { initial, deltaX } = e.detail;

		let minTop = sidebarStore.swipeMinimumTop;

		if (
			musicStore.listType === MusicListType.Album ||
			musicStore.listType === MusicListType.Music ||
			musicStore.listType === MusicListType.Folder
		) {
			minTop = (isMobile() ? mobileStore.statusBarHeight : 0) + filterStore.bar.height;
		}

		if (initial[1] < minTop) return;

		const swipeOpen =
			(type === SidebarType.Right && deltaX < -SWIPE_RANGE) ||
			(type === SidebarType.Left && deltaX > SWIPE_RANGE);

		const swipeClose =
			(type === SidebarType.Right && deltaX > SWIPE_RANGE) ||
			(type === SidebarType.Left && deltaX < -SWIPE_RANGE);

		if (swipeOpen && sidebarStore.showType === null) {
			isMouseInsideArea = true;
			isShowing = true;
			sidebarStore.showType = type;
		} else if (swipeClose && sidebarStore.showType === type) {
			setTimeout(() => {
				isMouseInsideArea = false;
				isShowing = false;
				sidebarStore.showType = null;
			});
		}
	}

	function onBodyMouseLeave(e: MouseEvent) {
		const onRightEdge = type === SidebarType.Right && e.clientX > window.innerWidth;
		const onLeftEdge = type === SidebarType.Left && e.clientX < 0;

		if(onRightEdge || onLeftEdge){
			isMouseInsideArea = true;
			isShowing = true;
			sidebarStore.showType = type;
		}
	}

	onMount(() => {
		sidebarStore.showType = null;
		setTimeout(() => (isMounted = true), 1500);
	});
</script>

<svelte:body use:swipeable on:swiped={onSwipe} onmouseleave={onBodyMouseLeave} />
<svelte:document onmousemove={onMouseMove} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="pointer-events-none fixed top-0 z-10 px-3
        {type === SidebarType.Right ? 'right-0' : 'left-0'}
        {isMounted ? '' : 'invisible'}"
	style="height: calc(100% - {playerBarStore.height}px - {paddingTop}px);
        top: {paddingTop}px;"
	onmouseleave={onMouseLeave}
>
	<!-- FIXME: Enable Sidebar animation on Linux. Probably won't happen -->
	<View
		class="linux-hardware-accelerate animate__animated pointer-events-auto h-full
			rounded-lg p-3
			{isShowing
			? type === SidebarType.Right
				? 'animate__fadeInRight'
				: 'animate__fadeInLeft'
			: type === SidebarType.Right
				? 'animate__fadeOutRight'
				: 'animate__fadeOutLeft'}
            {props.class}
		"
		style="
			width: {sidebarWidth - 24}px;
			animation-duration: {isLinux() ? '350ms' : '500ms'};
		"
	>
		{@render children?.()}
	</View>
</div>
