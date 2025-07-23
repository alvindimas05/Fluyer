<script lang="ts">
    import {IconThemeType, IconType} from "$lib/icon/types";
    import {iconRegistry} from "$lib/icon/registry/icon-registry";
    import type {IconWeight} from "phosphor-svelte";
    import {iconTheme} from "$lib/stores/icon";

    interface Props {
	type: IconType;
}

let { type }: Props = $props();
let Component = $derived(iconRegistry[$iconTheme]?.[type] ?? iconRegistry[$iconTheme]?.[IconType.Unknown]);
let color = $state("white");
let weight: IconWeight = $state("regular");
let classes = $state("");

function configureIcon(){

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
    switch ($iconTheme) {
        case IconThemeType.Lucide:
            classes = "!w-[90%]"
    }
}

$effect(configureIcon);
</script>

{#if Component}
    <div class="w-full h-full aspect-square m-auto {classes}">
        <Component class="w-full h-full text-[100%]" {color} {weight} />
    </div>
{:else}
    <span class="text-red-500">Icon not found</span>
{/if}