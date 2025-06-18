<script lang="ts">
import {IconThemeType, type IconType} from "$lib/icon/types";
import {iconRegistry} from "$lib/icon/registry";

interface Props {
    type: IconType,
}

let { type } = $props<Props>();
const themeType = IconThemeType.Phosphor;
const Component = iconRegistry[themeType]?.[type] ?? iconRegistry[themeType]?.['QuestionMark'];
</script>

{#if Component}
    <div class="w-full h-full aspect-square">
        <svelte:component class="w-full h-full text-[100%]" this={Component} />
    </div>
{:else}
    <span class="text-red-500">Icon not found</span>
{/if}