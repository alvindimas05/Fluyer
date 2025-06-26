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
	let isShowing = $state(false);

	function onMouseMove(e: MouseEvent) {
		if (
			((type === SidebarType.Right &&
				e.clientX > window.innerWidth - 20) ||
				(type === SidebarType.Left && e.clientX < 20)) &&
			e.clientY <= window.innerHeight - 8 * 16 &&
			!isMouseInsideArea
		) {
			isMouseInsideArea = true;
			isShowing = true;
		}
	}

	function onMouseLeave(e: MouseEvent) {
		if (!isMouseInsideArea ||
			(type === SidebarType.Right && e.clientX > window.innerWidth - 20) ||
			(type === SidebarType.Left && e.clientX < 20)) return;

		isShowing = false;
		isMouseInsideArea = false;
	}

	function onSwipe(e: CustomEvent<SwipeEventData>) {
		if (e.detail.initial[1] < $swipeMinimumTop && !isMouseInsideArea)
			return;
		if (
			((type === SidebarType.Right && e.detail.deltaX < -SWIPE_RANGE) ||
				(type === SidebarType.Left && e.detail.deltaX > SWIPE_RANGE)) &&
			$sidebarShowingType === null &&
			!isMouseInsideArea
		) {
			isMouseInsideArea = true;
			isShowing = true;
			$sidebarShowingType = type;
		} else if (
			((type === SidebarType.Right && e.detail.deltaX > SWIPE_RANGE) ||
				(type === SidebarType.Left &&
					e.detail.deltaX < -SWIPE_RANGE &&
					$sidebarShowingType !== null)) &&
			isMouseInsideArea
		) {
			setTimeout(() => {
				isShowing = false;
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
	transition-opacity duration-400 ease-in-out
	${isMouseInsideArea ? "opacity-100 pointer-events-auto" : "opacity-0 pointer-events-none"}`}
	style={`padding-top: ${(isMobile() ? $mobileStatusBarHeight : 0) + 44}px`}
	onmouseleave={onMouseLeave}
>
	<div
		class={`bg-gray-700 bg-opacity-30 backdrop-blur-sm md:backdrop-blur-md rounded shadow-xl
		text-white h-full w-full p-2 md:p-5
		${type === SidebarType.Right ? "fadeRight" : "fadeLeft"}
		${isShowing ? "show" : ""}`}
	>
		<div
			class="border border-gray-400 bg-white/5 h-full w-full rounded flex flex-col"
		>
			{@render children?.()}
		</div>
	</div>
</div>

<style lang="scss">
	.fadeRight,
	.fadeLeft {
		transition:
			transform 0.4s ease,
			opacity 0.4s ease;
		opacity: 0;
	}
	.fadeRight {
		transform: translateX(100%);
	}
	.fadeLeft {
		transform: translateX(-100%);
	}
	.fadeRight.show,
	.fadeLeft.show {
		transform: translateX(0%);
		opacity: 1;
	}
</style>
