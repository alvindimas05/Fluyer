<script lang="ts">
import Glass from "$lib/glass/Glass.svelte";

interface Props {
    children?: import("svelte").Snippet;
    class?: string;
    style?: string;
    thisElement?: HTMLDivElement;
    glassEnableBlur?: boolean;
    glassEnableHoverEffect?: boolean;
    events: any;
}

let {
    glassEnableBlur = false,
    glassEnableHoverEffect = false,
    children,
    thisElement = $bindable<HTMLDivElement>(),
    ...props
}: Props = $props();
</script>

<Glass class="bg-gray-300/10 {glassEnableHoverEffect && 'hover:bg-gray-200/20'} {props.class}"
    style={props.style}
    enableBlur={glassEnableBlur}
    bind:thisElement={thisElement}
    events={props.events}>
    {@render children?.()}
</Glass>