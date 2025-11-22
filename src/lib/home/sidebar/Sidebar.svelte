<script lang="ts">
import { SidebarType } from "$lib/home/sidebar/types";

interface Props {
	children?: import("svelte").Snippet;
	type: SidebarType;
	class?: string;
}

const props = $props();
let { children, type }: Props = props;

import { isLinux, isMobile, isWindows } from "$lib/platform";
import { swipeable } from "@react2svelte/swipeable";
import type { SwipeEventData } from "@react2svelte/swipeable";
import { swipeMinimumTop } from "$lib/stores";
import { onMount } from "svelte";
import { playerBarHeight } from "$lib/stores/playerbar.svelte";
import { filterBarHeight } from "$lib/stores/filterbar";
import { getCurrentWindow } from "@tauri-apps/api/window";
import View from "$lib/ui/components/View.svelte";

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
	[640, 0, 0.33334], // sm → 33.3334%
];

const SWIPE_RANGE = 125;

let sidebarWidth = $state(window.innerWidth);
let paddingTop = $derived(
	(isMobile() ? $mobileStatusBarHeight : 0) + $filterBarHeight,
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

	const onLeftEdge =
		type === SidebarType.Left && e.clientX < (isMaximized ? 4 : 12);

	const withinVerticalBounds = e.clientY <= window.innerHeight - 8 * 16;

	if (
		(onRightEdge || onLeftEdge) &&
		withinVerticalBounds &&
		!isMouseInsideArea
	) {
		isMouseInsideArea = true;
		isShowing = true;
		$sidebarShowingType = type;
	}
}

async function onMouseLeave(e: MouseEvent) {
	const nearScreenEdge = e.clientX <= 20 || e.clientX >= window.innerWidth - 20;

	if (!isMouseInsideArea || nearScreenEdge) return;

	isShowing = false;
	isMouseInsideArea = false;
	$sidebarShowingType = null;
}

function onSwipe(e: CustomEvent<SwipeEventData>) {
	const { initial, deltaX } = e.detail;
	if (initial[1] < $swipeMinimumTop) return;

	const swipeOpen =
		(type === SidebarType.Right && deltaX < -SWIPE_RANGE) ||
		(type === SidebarType.Left && deltaX > SWIPE_RANGE);

	const swipeClose =
		(type === SidebarType.Right && deltaX > SWIPE_RANGE) ||
		(type === SidebarType.Left && deltaX < -SWIPE_RANGE);

	if (swipeOpen && $sidebarShowingType === null) {
		isMouseInsideArea = true;
		isShowing = true;
		$sidebarShowingType = type;
	} else if (swipeClose && $sidebarShowingType === type) {
		setTimeout(() => {
			isMouseInsideArea = false;
			isShowing = false;
			$sidebarShowingType = null;
		});
	}
}

function updateSidebarWidth() {
	const w = window.innerWidth;
	const dpi = window.devicePixelRatio;

	for (const [minW, minDppx, width] of rules) {
		if (w >= minW && dpi >= minDppx) {
			const columnPercentage = width * window.innerWidth;
			// 2 columns size
			sidebarWidth = columnPercentage * 2;
			return;
		}
	}
	sidebarWidth = window.innerWidth;
}

function updateSize() {
	updateSidebarWidth();
}

onMount(() => {
	updateSize();
	$sidebarShowingType = null;
	setTimeout(() => (isMounted = true), 750);
});
</script>

<svelte:window onresize={updateSize} />
<svelte:body use:swipeable on:swiped={onSwipe} />
<svelte:document onmousemove={onMouseMove} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed top-0 z-10 px-3 pointer-events-none
	{type === SidebarType.Right ? 'right-0' : 'left-0'}
	{isMounted ? '' : 'invisible'}"
	style="height: calc(100% - {$playerBarHeight}px - {paddingTop}px);
	top: {paddingTop}px;"
	onmouseleave={onMouseLeave}>
	<View glassEnableBlur={true}
        class="bg-gray-400/35 pointer-events-auto h-full p-3 rounded-lg
			animate__animated
			{isShowing
				? (type === SidebarType.Right
					? 'animate__slideInRight'
					: 'animate__slideInLeft')
					: (type === SidebarType.Right
					? 'animate__slideOutRight'
					: 'animate__slideOutLeft')}
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