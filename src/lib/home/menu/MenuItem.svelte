<script lang="ts">
import type { IconType } from "$lib/icon/types";
import Icon from "$lib/icon/Icon.svelte";
import Glass from "$lib/glass/Glass.svelte";

interface Props {
	onclick: () => void;
	icon: IconType;
	label: string;
}

const { onclick, icon, label }: Props = $props();
</script>
<Glass class="mx-2 my-3 cursor-pointer hover:bg-white/10"
    padding="12px"
    paddingHover="16px"
    events={{
        onclick
    }}>
    <button
        class="w-full text-white text-start font-medium tracking-wide px-1
        text-base md:text-lg grid grid-cols-[min-content_auto] gap-3 items-center"
    >
        <div class="w-6"><Icon type={icon}/></div>
        <div class="text-start">{label}</div>
    </button>
</Glass>