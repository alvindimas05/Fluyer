<script lang="ts">
import Glass from "$lib/glass/Glass.svelte";

interface Props {
    children?: import("svelte").Snippet;
    class?: string;
    onclick?: (event: MouseEvent & {
        currentTarget: EventTarget & HTMLDivElement;
    }) => void;
}

const props: Props = $props();
</script>

<Glass class={props.class}
    onclick={props.onclick}>
    {@render props.children?.()}
</Glass>