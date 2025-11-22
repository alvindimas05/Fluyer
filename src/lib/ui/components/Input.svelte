<script lang="ts">
import { IconType } from "$lib/icon/types.js";
import Icon from "$lib/icon/Icon.svelte";
import View from "$lib/ui/components/View.svelte";

interface Props {
	class?: string;
	value?: string;
	placeholder?: string;
	icon?: IconType;
}

let { value = $bindable(), icon, ...props }: Props = $props();

let isPressed = $state(false);
let inputElement: HTMLInputElement;

const handleClick = (
	event: MouseEvent & { currentTarget: EventTarget & HTMLDivElement },
) => {
	isPressed = true;

	setTimeout(() => (isPressed = false), 150);
	inputElement.focus();
};
</script>

<View class="{props.class} {isPressed ? 'scale-[.99]' : 'scale-100'}"
    glassEnableHoverEffect={true}
    events={{
        onclick: handleClick,
        ontouchstart: handleClick,
    }}>
    <div class="w-full h-full grid items-center cursor-text px-2 py-[6px]
        {icon && 'grid-cols-[auto_min-content]'}">
        <input
                class="w-full h-full bg-transparent placeholder:text-white/70 outline-none text-sm"
                placeholder={props.placeholder}
                bind:value={value}
                bind:this={inputElement}
        />
        {#if icon}
            <div class="w-4">
                <Icon type={icon} />
            </div>
        {/if}
    </div>
</View>