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
            class="w-full text-white text-start font-medium tracking-wide p-3 rounded-lg
    bg-white/15 hover:bg-white/25 transition-all duration-200 box-border
    shadow-md md:text-lg grid grid-cols-[min-content_auto] gap-3 items-center"
            onclick={onclick}
    >
        <div class="w-6"><Icon type={icon}/></div>
        <div class="text-start">{label}</div>
    </button>
</div>