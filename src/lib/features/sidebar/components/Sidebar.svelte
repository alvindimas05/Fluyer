<script lang="ts">
	import { SidebarType } from '$lib/features/sidebar/types';

	interface Props {
		children?: import('svelte').Snippet;
		type: SidebarType;
		class?: string;
	}

	interface MouseLeavePayload {
		x: number;
		y: number;
	}

	const props = $props();
	let { children, type }: Props = props;

	import { isLinux, isMobile, isWindows } from '$lib/platform';
	// import { swipeable } from '@react2svelte/swipeable';
	// import type { SwipeEventData } from '@react2svelte/swipeable';
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import View from '$lib/ui/components/View.svelte';
	import playerBarStore from '$lib/stores/playerbar.svelte';
	import sidebarStore from '$lib/stores/sidebar.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import { filterBarStore } from '$lib/stores/filter.svelte';

	import { MusicListType } from '$lib/features/music/types';
	import musicStore from '$lib/stores/music.svelte';
	import { CommandRoutes } from '$lib/constants/CommandRoutes';
	import modalStore from '$lib/stores/modal.svelte';

	// const SWIPE_RANGE_X = 125;
	// const SWIPE_RANGE_Y = 50;

	const currentWindow = getCurrentWindow();

	let sidebarWidth = $derived(sidebarStore.width);
	let paddingTop = $derived((isMobile() ? mobileStore.statusBarHeight : 0) + filterBarStore.height);

	let isMouseInsideArea = $state(false);
	let isShowing = $state(false);
	let isMaximized = $state(true);
	let isRendered = $state(false);

	$effect(() => {
		if (isShowing) {
			isRendered = true;
		}
	});

	$effect(() => {
		const show = sidebarStore.showType !== null && sidebarStore.showType === type;
		isMouseInsideArea = show;
		isShowing = show;
	});

	function handleAnimationEnd() {
		if (!isShowing) {
			isRendered = false;
		}
	}

	async function onMouseMove(e: MouseEvent) {
		if (modalStore.show || isMobile()) return;
		// Ignore while user is holding mouse button (e.g. dragging)
		if (e.buttons !== 0) return;

		const onRightEdge =
			type === SidebarType.Right &&
			e.clientX > window.innerWidth - (isWindows() || !isMaximized ? 12 : 4);

		const onLeftEdge = type === SidebarType.Left && e.clientX < (isMaximized ? 4 : 12);

		const withinVerticalBounds = e.clientY <= window.innerHeight - 8 * 16;

		if ((onRightEdge || onLeftEdge) && withinVerticalBounds && !isMouseInsideArea) {
			sidebarStore.showType = type;
		} else if (isMouseInsideArea) {
			const sidebarLeft = type === SidebarType.Right ? window.innerWidth - sidebarWidth : 0;
			const sidebarRight = type === SidebarType.Right ? window.innerWidth : sidebarWidth;
			const sidebarTop = paddingTop;
			const sidebarBottom = window.innerHeight - playerBarStore.height;

			const isOutside =
				e.clientX < sidebarLeft ||
				e.clientX > sidebarRight ||
				e.clientY < sidebarTop ||
				e.clientY > sidebarBottom;

			if (isOutside) {
				onMouseLeave(e);
			}
		}
	}

	async function onMouseLeave(e: MouseEvent) {
		const nearScreenEdge = e.clientX <= 20 || e.clientX >= window.innerWidth - 20;

		if (!isMouseInsideArea || nearScreenEdge) return;
		// Don't close sidebar while user is holding mouse button (dragging)
		if (e.buttons !== 0) return;

		sidebarStore.showType = null;
	}

	/*
	function onSwipe(e: CustomEvent<SwipeEventData>) {
		if (modalStore.show) return;

		const { initial, deltaX, deltaY } = e.detail;

		let minTop = sidebarStore.swipeMinimumTop;

		if (
			musicStore.listType === MusicListType.Album ||
			musicStore.listType === MusicListType.Music ||
			musicStore.listType === MusicListType.Folder
		) {
			minTop = (isMobile() ? mobileStore.statusBarHeight : 0) + filterBarStore.height;
		}

		if (initial[1] < minTop) return;

		const swipeOpen =
			(type === SidebarType.Right && deltaX < -SWIPE_RANGE_X) ||
			(type === SidebarType.Left && deltaX > SWIPE_RANGE_X);

		const swipeClose =
			(type === SidebarType.Right && deltaX > SWIPE_RANGE_X) ||
			(type === SidebarType.Left && deltaX < -SWIPE_RANGE_X);

		const swipeIsNotVertical = Math.abs(deltaY) < SWIPE_RANGE_Y;

		if (swipeOpen && sidebarStore.showType === null && swipeIsNotVertical) {
			isMouseInsideArea = true;
			isShowing = true;
			sidebarStore.showType = type;
		} else if (swipeClose && sidebarStore.showType === type && swipeIsNotVertical) {
			setTimeout(() => {
				isMouseInsideArea = false;
				isShowing = false;
				sidebarStore.showType = null;
			});
		}
	}
	*/

	function onBodyMouseLeave(e: MouseEvent) {
		if (isMobile() || isLinux()) return;

		const onRightEdge = type === SidebarType.Right && e.clientX > window.innerWidth;
		const onLeftEdge = type === SidebarType.Left && e.clientX < 0;

		if (onRightEdge || onLeftEdge) {
			isMouseInsideArea = true;
			isShowing = true;
			sidebarStore.showType = type;
		}
	}

	currentWindow.onResized(async () => {
		isMaximized = await currentWindow.isMaximized();
	});

	onMount(() => {
		sidebarStore.showType = null;
		let unlistenLinuxMouseLeave: UnlistenFn | null = null;

		if (isLinux()) {
			listen<MouseLeavePayload>(CommandRoutes.SIDEBAR_MOUSE_LEAVE, (e) => {
				const x = e.payload.x;
				const y = e.payload.y;
				const slicedWidth = 0.25 * window.innerWidth;
				const slicedHeight = 0.05 * window.innerHeight;

				if (y < slicedHeight) return;

				const onRightEdge = type === SidebarType.Right && x >= window.innerWidth - slicedWidth;
				const onLeftEdge = type === SidebarType.Left && x <= slicedWidth;

				if (onRightEdge || onLeftEdge) {
					isMouseInsideArea = true;
					isShowing = true;
					sidebarStore.showType = type;
				}
			}).then((unlisten) => {
				unlistenLinuxMouseLeave = unlisten;
			});
		}

		return () => {
			if (unlistenLinuxMouseLeave) {
				unlistenLinuxMouseLeave();
			}
		};
	});
</script>

<svelte:body onmouseleave={onBodyMouseLeave} />
<!-- <svelte:body use:swipeable on:swiped={onSwipe} onmouseleave={onBodyMouseLeave} /> -->
<svelte:document onmousemove={onMouseMove} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if isRendered}
	<div
		class="pointer-events-none fixed top-0 z-10 px-3
			{type === SidebarType.Right ? 'right-0' : 'left-0'}"
		style="height: calc(100% - {playerBarStore.height}px - {paddingTop}px);
			top: {paddingTop}px;"
		onmouseleave={onMouseLeave}
	>
		<View
			class="animate__animated pointer-events-auto h-full
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
			events={{ onanimationend: handleAnimationEnd }}
		>
			{@render children?.()}
		</View>
	</div>
{/if}
