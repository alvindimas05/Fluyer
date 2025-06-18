<script lang="ts">
import {IconThemeType, type IconType} from "$lib/icon/types";
import {iconRegistry} from "$lib/icon/registry";
import type {IconWeight} from "phosphor-svelte";

interface Props {
    type: IconType,
    color?: string,
    weight?: IconWeight,
}

let { type, color = "white", weight = "regular" } = $props<Props>();
const themeType = IconThemeType.Phosphor;
const Component = iconRegistry[themeType]?.[type] ?? iconRegistry[themeType]?.['QuestionMark'];
</script>

{#if Component}
    <i class="w-full h-full aspect-square">
        <svelte:component class="w-full h-full text-[100%]" {color} {weight} this={Component} />
    </i>
{:else}
    <span class="text-red-500">Icon not found</span>
{/if}