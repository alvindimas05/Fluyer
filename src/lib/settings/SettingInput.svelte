<script lang="ts">
interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>
<div class="bg-gradient-to-r from-white/15 to-white/10 rounded shadow-md my-3">
    {@render children?.()}
</div>