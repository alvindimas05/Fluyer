<script lang="ts">
	import type { IconType } from '$lib/icon/types';
	import Button from '$lib/ui/components/Button.svelte';
	import Icon from '$lib/ui/icon/Icon.svelte';

	interface Props {
		onclick: () => void;
		icon: IconType;
		label: string;
	}

	const { onclick, icon, label }: Props = $props();
</script>

<Button
	class="mx-2 my-3 grid cursor-pointer grid-cols-[min-content_auto]
    items-center gap-3 rounded-md px-3 py-2 text-base font-medium
    tracking-wide hover:px-[15px] hover:py-[11px] md:text-lg"
	{onclick}
>
	<div class="w-6"><Icon type={icon} /></div>
	<div class="text-start">{label}</div>
</Button>
