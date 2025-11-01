<script lang="ts">
import { IconThemeType, IconType } from "$lib/icon/types";
import { iconRegistry } from "$lib/icon/registry/icon-registry";
import type { IconWeight } from "phosphor-svelte";
import { iconTheme } from "$lib/stores/icon";

interface Props {
	type: IconType;
	class?: string;
	style?: string;
}

const props = $props();
let { type }: Props = props;
let Component = $derived(
	iconRegistry[$iconTheme]?.[type] ??
		iconRegistry[$iconTheme]?.[IconType.Unknown],
);
let color = $state("white");
let weight: IconWeight = $state("regular");
let classes = $state("");
let iconSize = $state(100);

function configureIcon() {
	color = "white";
	weight = "regular";
	classes = "";

	switch (type) {
		case IconType.Trash:
			weight = "fill";
			color = "rgb(255, 150, 150)";
			break;
		case IconType.Note:
			weight = "bold";
			break;
	}
}

$effect(configureIcon);
</script>

{#if Component}
    <div class="w-full h-full aspect-square m-auto {classes} {props.class}"
        style={props.style}>
        <Component class="w-full h-full text-[100%] {$iconTheme === IconThemeType.Lucide && 'p-[10%]'}" {color} {weight} />
    </div>
{:else}
    <span class="text-red-500">Icon not found</span>
{/if}