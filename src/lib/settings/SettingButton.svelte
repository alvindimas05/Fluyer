<script lang="ts">
import type { IconType } from "$lib/icon/types";
import Icon from "$lib/icon/Icon.svelte";
import Button from "$lib/ui/components/Button.svelte";

interface Props {
	onclick?: () => void;
	icon?: IconType;
	label: string;
}

const { onclick, label, icon }: Props = $props();
</script>
<div class="w-full px-5">
    <Button class="w-full text-start px-3 py-2 my-3 rounded
        {icon ? 'grid items-center grid-cols-[min-content_auto] gap-2' : ''}"
        glassShineSize="sm"
        onclick={onclick}
    >
        {#if icon}
            <div class="w-5"><Icon type={icon}/></div>
        {/if}
        <div class="text-start">{label}</div>
    </Button>
</div>