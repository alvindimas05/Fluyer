<script lang="ts">
import { SidebarType } from "$lib/home/sidebar/types";

interface Props {
	children?: import("svelte").Snippet;
	type: SidebarType;
}

let { children, type }: Props = $props();

import { isMobile } from "$lib/platform";
import { swipeable } from "@react2svelte/swipeable";
import type { SwipeEventData } from "@react2svelte/swipeable";
import { swipeMinimumTop } from "$lib/stores";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
import { sidebarShowingType } from "$lib/stores/sidebar";
import { onMount } from "svelte";

const SWIPE_RANGE = 125;

let isMouseInsideArea = $state(false);
let animationClass = $state("");

function onMouseMove(
	e: MouseEvent & {
		currentTarget: EventTarget & Document;
	},
) {
	if (isMobile()) return;
	if (
		((type === SidebarType.Right && e.clientX > window.innerWidth - 20) ||
			(type === SidebarType.Left && e.clientX < 20)) &&
		e.clientY <= window.innerHeight - 8 * 16 &&
		!isMouseInsideArea
	) {
		isMouseInsideArea = true;
		animationClass =
			type === SidebarType.Right
				? "animate__fadeInRight"
				: "animate__fadeInLeft";
	}
}

function onMouseLeave(
	e: MouseEvent & {
		currentTarget: EventTarget & HTMLDivElement;
	},
) {
	if (
		!isMouseInsideArea ||
		(type === SidebarType.Right && e.clientX >= window.innerWidth) ||
		(type === SidebarType.Left && e.clientX <= 0)
	)
		return;
	animationClass =
		type === SidebarType.Right
			? "animate__fadeOutRight"
			: "animate__fadeOutLeft";
}

function onAnimationEnd() {
	if (
		(type === SidebarType.Right && animationClass != "animate__fadeOutRight") ||
		(type === SidebarType.Left && animationClass != "animate__fadeOutLeft")
	)
		return;
	isMouseInsideArea = false;
}

function onSwipe(e: CustomEvent<SwipeEventData>) {
	if (
		!isMobile() ||
		(e.detail.initial[1] < $swipeMinimumTop && !isMouseInsideArea)
	)
		return;
	if (
		((type === SidebarType.Right && e.detail.deltaX < -SWIPE_RANGE) ||
			(type === SidebarType.Left && e.detail.deltaX > SWIPE_RANGE)) &&
		$sidebarShowingType === null &&
		!isMouseInsideArea
	) {
		isMouseInsideArea = true;
		animationClass =
			type === SidebarType.Right
				? "animate__fadeInRight"
				: "animate__fadeInLeft";
		$sidebarShowingType = type;
	} else if (
		((type === SidebarType.Right && e.detail.deltaX > SWIPE_RANGE) ||
			(type === SidebarType.Left &&
				e.detail.deltaX < -SWIPE_RANGE &&
				$sidebarShowingType !== null)) &&
		isMouseInsideArea
	) {
		setTimeout(() => {
			animationClass =
				type === SidebarType.Right
					? "animate__fadeOutRight"
					: "animate__fadeOutLeft";
			$sidebarShowingType = null;
		}, 0);
	}
}

onMount(() => {
	$sidebarShowingType = null;
});
</script>

<svelte:body use:swipeable on:swiped={onSwipe} />
<svelte:document onmousemove={onMouseMove} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class={`fixed ${type === SidebarType.Right ? "right-0" : "left-0"} top-0 z-10 h-[calc(100%-8rem)]
    w-full px-3 sm:w-[70%]
    md-mdpi:w-[50%] lg-mdpi:w-[40%] xl-mdpi:w-[30%]
    md-hdpi:w-[50%] lg-hdpi:w-[35%]
	${isMouseInsideArea ? "" : "hidden"}`}
    style={`padding-top: ${(isMobile() ? $mobileStatusBarHeight : 0) + 44}px`}
    onmouseleave={onMouseLeave}
>
    <div
        class={`bg-gray-700 bg-opacity-30 backdrop-blur-md rounded shadow-xl
    text-white h-full w-full p-5 animate__animated animate__faster
    ${animationClass}`}
        onanimationend={onAnimationEnd}
    >
        <div
            class="border border-gray-400 bg-white/5 h-full w-full rounded flex flex-col"
        >
            {@render children?.()}
        </div>
    </div>
</div>
