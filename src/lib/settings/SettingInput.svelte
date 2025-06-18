<script lang="ts">
interface Props {
    children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>
<div class="w-full bg-white/10 text-white my-3 rounded-md">
    {@render children?.()}
</div>