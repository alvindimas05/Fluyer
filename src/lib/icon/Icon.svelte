<script lang="ts">
import {IconThemeType, IconType} from "$lib/icon/types";
import {iconRegistry} from "$lib/icon/registry";
import type {IconWeight} from "phosphor-svelte";

interface Props {
    type: IconType,
}

let {type}: Props = $props();
const themeType = IconThemeType.Phosphor;
const Component = iconRegistry[themeType]?.[type] ?? iconRegistry[themeType]?.['QuestionMark'];

let color = "white";
let weight: IconWeight = "regular";

switch(themeType){
    case IconThemeType.Phosphor:
        switch(type){
            case IconType.Trash:
                weight = "fill";
                color = "rgb(255, 150, 150)";
                break;
            case IconType.Note:
                weight = "bold";
                break;
        }
        break;
}
</script>

{#if Component}
    <i class="w-full h-full aspect-square">
        <svelte:component class="w-full h-full text-[100%]" {color} {weight} this={Component} />
    </i>
{:else}
    <span class="text-red-500">Icon not found</span>
{/if}