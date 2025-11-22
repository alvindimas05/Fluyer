<script lang="ts">
import View from "$lib/ui/components/View.svelte";

interface Props {
	children?: import("svelte").Snippet;
}

let { children }: Props = $props();
</script>
<div class="w-full px-5">
    <View class="w-full my-3 rounded"
        glassShineSize="sm">
        {@render children?.()}
    </View>
</div>