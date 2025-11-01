<script lang="ts">
import Glass from "$lib/glass/Glass.svelte";
import View from "$lib/components/View.svelte";

interface Props {
	children?: import("svelte").Snippet;
	class?: string;
	onclick?: (
		event: MouseEvent & {
			currentTarget: EventTarget & HTMLDivElement;
		},
	) => void;
}

const props: Props = $props();

let isPressed = $state(false);

const handleClick = (
	event: MouseEvent & { currentTarget: EventTarget & HTMLDivElement },
) => {
	isPressed = true;

	setTimeout(() => (isPressed = false), 100);

	setTimeout(() => props.onclick && props.onclick(event), 200);
};
</script>

<View
    class="cursor-pointer {props.class} {isPressed ? 'scale-95' : 'scale-100'}"
    glassEnableHoverEffect={true}
    events={{
        onclick: handleClick,
    }}>
    {@render props.children?.()}
</View>