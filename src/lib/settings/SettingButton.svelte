<script lang="ts">
import type { IconType } from "$lib/icon/types";
import Icon from "$lib/icon/Icon.svelte";

interface Props {
	onclick?: () => void;
	icon?: IconType;
	label: string;
}

const { onclick, label, icon }: Props = $props();
</script>
<button
        class="w-full text-white text-start px-3 py-2 my-3
        bg-gradient-to-r from-white/15 to-white/10 rounded shadow-md
        hover:from-white/25 hover:to-white/30
        focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200
        {icon ? 'grid items-center grid-cols-[min-content_auto] gap-2' : ''}"
        onclick={onclick}
>
    {#if icon}
        <div class="w-5"><Icon type={icon}/></div>
    {/if}
    <div class="text-start">{label}</div>
</button>