<script lang="ts">
interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>
<div class="w-full px-5">
    <div class="w-full bg-white/5 border border-white/30 rounded-lg shadow-md my-3">
        {@render children?.()}
    </div>
</div>