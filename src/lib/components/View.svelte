<script lang="ts">
import Glass from "$lib/glass/Glass.svelte";

interface Props {
    children?: import("svelte").Snippet;
    class?: string;
    style?: string;
    thisElement?: HTMLDivElement;
    glassEnableBlur?: boolean;
}

let {
    glassEnableBlur = false,
    children,
    thisElement = $bindable<HTMLDivElement>(),
    ...props
}: Props = $props();
</script>

<Glass class="bg-gray-400/20 {props.class}" style={props.style} enableBlur={glassEnableBlur} bind:thisElement={thisElement}>
    {@render children?.()}
</Glass>