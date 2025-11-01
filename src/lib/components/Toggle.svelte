<script lang="ts">
import type { IconType } from "$lib/icon/types";
import Glass from "$lib/glass/Glass.svelte";
import { onDestroy, onMount } from "svelte";
import Icon from "$lib/icon/Icon.svelte";
import View from "$lib/components/View.svelte";

interface Props {
	class?: string;
	checked?: boolean;
	onchange?: (checked: boolean) => void;
	checkedIcon?: IconType;
	uncheckedIcon?: IconType;
}

const props = $props();
const { onchange, checkedIcon, uncheckedIcon }: Props = props;

let toggleElement: HTMLDivElement;
let toggleObserver: ResizeObserver;
let checked = $state(props.checked);
let toggleHeight = $state(0);
let translateX = $state(0);

function handleChange(
	e: Event & {
		currentTarget: HTMLInputElement;
	},
) {
	onchange?.(checked);
}

function updateProperties() {
	if (!toggleElement) return;
	toggleHeight = toggleElement.offsetHeight - 8; // Account for 4px padding on each side
	translateX = toggleElement.offsetWidth - toggleHeight - 8; // Total width - thumb size - padding
}

function listenResize() {
	if (!toggleElement) return;
	toggleObserver = new ResizeObserver(updateProperties);
	toggleObserver.observe(toggleElement);
}

let isPressed = $state(false);

const handleClick = (
	event: MouseEvent & { currentTarget: EventTarget & HTMLDivElement },
) => {
	isPressed = true;

	setTimeout(() => (isPressed = false), 150);
};

onMount(() => {
	setTimeout(updateProperties, 10);
	listenResize();
});
onDestroy(() => {
	toggleObserver?.disconnect();
});
</script>

<View class="rounded {isPressed ? 'scale-95' : 'scale-100'} {props.class}"
    glassEnableHoverEffect={true}
    bind:thisElement={toggleElement}
    events={{
        onclick: handleClick,
        ontouchstart: handleClick,
    }}>
    <label class="w-full h-full relative inline-flex items-center cursor-pointer">
        <input class="sr-only" type="checkbox" onchange={handleChange} bind:checked />
        <!-- Toggle thumb with icon -->
        <div class="absolute top-1 left-1 rounded-full
            flex items-center justify-center px-[2px]
            transition-all duration-500 pointer-events-none"
            style="width: {toggleHeight}px;
                height: {toggleHeight}px;
                transform: translateX({checked ? translateX : 0}px);">
            {#if checked}
                <Icon type={checkedIcon} />
            {:else}
                <Icon type={uncheckedIcon} />
            {/if}
        </div>
    </label>
</View>