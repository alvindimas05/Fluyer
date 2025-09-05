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
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import { sidebarShowingType } from "$lib/stores/sidebar";
import { onMount } from "svelte";
import Glass from "$lib/glass/Glass.svelte";
import {playerBarHeight} from "$lib/stores/playerbar";
import {filterBarHeight} from "$lib/stores/filterbar";
    import { getCurrentWindow } from "@tauri-apps/api/window";

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
let paddingTop = $derived((isMobile() ? $mobileStatusBarHeight : 0) + $filterBarHeight);

let isMouseInsideArea = $state(false);
let isShowing = $state(false);
let isMounted = $state(false);

async function onMouseMove(e: MouseEvent) {
	if (
		((type === SidebarType.Right && e.clientX > window.innerWidth - (isWindows() || !(await getCurrentWindow().isMaximized()) ? 12 : 4)) ||
			(type === SidebarType.Left && e.clientX < (await getCurrentWindow().isMaximized() ? 4 : 12))) &&
		e.clientY <= window.innerHeight - 8 * 16 &&
		!isMouseInsideArea
	) {
		isMouseInsideArea = true;
		isShowing = true;
		$sidebarShowingType = type;
	}
}

async function onMouseLeave(e: MouseEvent) {
	if (
		!isMouseInsideArea || (e.clientX > window.innerWidth - 20 || e.clientX < 20)
	)
		return;
	isShowing = false;
	isMouseInsideArea = false;
	$sidebarShowingType = null;
}

function onSwipe(e: CustomEvent<SwipeEventData>) {
	if (e.detail.initial[1] < $swipeMinimumTop) return;

	if (
		((type === SidebarType.Right && e.detail.deltaX < -SWIPE_RANGE) ||
			(type === SidebarType.Left && e.detail.deltaX > SWIPE_RANGE)) &&
		$sidebarShowingType === null
	) {
		isMouseInsideArea = true;
		isShowing = true;
		$sidebarShowingType = type;
	} else if (
		((type === SidebarType.Right && e.detail.deltaX > SWIPE_RANGE) ||
			(type === SidebarType.Left && e.detail.deltaX < -SWIPE_RANGE)) &&
		$sidebarShowingType === type
	) {
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
	setTimeout(() => isMounted = true, 750);
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
	<Glass enableHoverAnimation={false} enableBlur={true}
		glassEffectScale={50}
		class="
			h-full p-3 !rounded-md pointer-events-auto
			animate__animated
			{isShowing
				? (type === SidebarType.Right
					? 'animate__slideInRight'
					: 'animate__slideInLeft')
				: (type === SidebarType.Right
					? 'animate__slideOutRight'
					: 'animate__slideOutLeft')}
		"
		wrapperClass="!rounded-md {props.class}"
		style="
			width: {sidebarWidth - 24}px;
			animation-duration: {isLinux() ? '350ms' : '500ms'};
		"
	>
		{@render children?.()}
	</Glass>
</div>
