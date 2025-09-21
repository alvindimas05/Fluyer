<script lang="ts">
import Glass from "$lib/glass/Glass.svelte";

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>
<Glass class="my-3 mx-5 !rounded-lg" wrapperClass="!rounded-lg" padding="0">
    {@render children?.()}
</Glass>