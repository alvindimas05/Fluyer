<script lang="ts">
import type { IconType } from "$lib/icon/types";
import Icon from "$lib/icon/Icon.svelte";

interface Props {
	onclick: () => void;
	icon: IconType;
	label: string;
}

const { onclick, icon, label }: Props = $props();
</script>
<div class="w-full py-1 px-3">
    <button
            class="w-full text-white text-start font-medium tracking-wide px-4 py-3 rounded
    bg-gradient-to-r from-white/20 to-white/30 hover:from-white/30 hover:to-white/35
    focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200 box-border
    shadow-md md:text-lg grid grid-cols-[min-content_auto] gap-3 items-center"
            onclick={onclick}
    >
        <div class="w-6"><Icon type={icon}/></div>
        <div class="text-start">{label}</div>
    </button>
</div>