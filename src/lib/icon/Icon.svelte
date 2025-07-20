<script lang="ts">
    import {IconThemeType, IconType} from "$lib/icon/types";
    import {iconRegistry} from "$lib/icon/registry/icon-registry";
    import type {IconWeight} from "phosphor-svelte";

    interface Props {
	type: IconType;
}

let { type }: Props = $props();
const themeType: IconThemeType = IconThemeType.Material;
const Component =
	iconRegistry[themeType]?.[type] ?? iconRegistry[themeType]?.[IconType.Unknown];

let color = "white";
let weight: IconWeight = "regular";
let classes = "";

switch (type) {
    case IconType.Trash:
        weight = "fill";
        color = "rgb(255, 150, 150)";
        break;
    case IconType.Note:
        weight = "bold";
        break;
}

switch (themeType) {
    case IconThemeType.Lucide:
        switch(type){
            case IconType.Play:
            case IconType.Pause:
            case IconType.Next:
            case IconType.Previous:
            case IconType.Playing:
                classes = "w-[90%]"
        }
}
</script>

{#if Component}
    <div class="w-full h-full aspect-square m-auto {classes}">
        <Component class="w-full h-full text-[100%]" {color} {weight} />
    </div>
{:else}
    <span class="text-red-500">Icon not found</span>
{/if}