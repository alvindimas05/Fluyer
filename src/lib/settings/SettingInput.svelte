<script lang="ts">
interface Props {
    children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>
<div class="bg-white/5 border border-white/30 rounded-lg shadow-md my-3 mx-5">
    {@render children?.()}
</div>