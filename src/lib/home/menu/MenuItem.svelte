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
<button
        class="text-white text-start font-medium tracking-wide px-4 py-3 my-1 mx-3 rounded
        bg-gradient-to-r from-white/20 to-white/30 hover:from-white/30 hover:to-white/35
        focus:outline-none focus:ring-2 focus:ring-white/30 transition-all duration-200
        shadow-md hover:shadow-lg md:text-lg backdrop-blur-md grid grid-cols-[min-content_auto] gap-3 items-center"
        onclick={onclick}
    >
        <div class="w-6"><Icon type={icon}/></div>
        <div class="text-start">{label}</div>
    </button>