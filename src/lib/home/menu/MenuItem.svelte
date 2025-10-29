<script lang="ts">
import type { IconType } from "$lib/icon/types";
import Icon from "$lib/icon/Icon.svelte";
import Button from "$lib/components/Button.svelte";

interface Props {
	onclick: () => void;
	icon: IconType;
	label: string;
}

const { onclick, icon, label }: Props = $props();
</script>

<Button class="mx-2 my-3 cursor-pointer font-medium tracking-wide
    text-base md:text-lg grid grid-cols-[min-content_auto] gap-3 items-center rounded-md
    px-3 py-2 hover:px-[15px] hover:py-[11px]"
    {onclick}>
    <div class="w-6"><Icon type={icon}/></div>
    <div class="text-start">{label}</div>
</Button>