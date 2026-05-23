<script lang="ts">
	import { type IToast } from '$lib/services/ToastService.svelte';
	import ToastService from '$lib/services/ToastService.svelte';
	import View from '../components/View.svelte';

	interface Props {
		toast: IToast;
	}

	let { toast }: Props = $props();

	function close() {
		ToastService.remove(toast.id);
	}
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<View
	class="pointer-events-auto relative mb-3 w-full cursor-pointer items-center justify-between overflow-hidden rounded px-3 py-2 transition-all"
	events={{
		onclick: close
	}}
>
	<div>{toast.message}</div>
	{#if toast.action}
		<div class="mt-2 flex shrink-0 gap-2">
			{#each Array.isArray(toast.action) ? toast.action : [toast.action] as action}
				<button
					class="rounded bg-white/20 px-2 py-1 text-sm font-semibold hover:bg-white/30"
					onclick={(e) => {
						e.stopPropagation();
						action.onClick();
						close();
					}}
				>
					{action.label}
				</button>
			{/each}
		</div>
	{/if}
</View>

<style>
	/* Tailwind utilities used in classes, custom colors handled via utility classes or inline styles if specific opacity needed */
	.border-white-20 {
		border-color: rgba(255, 255, 255, 0.2);
	}
	.bg-red-500-10 {
		background-color: rgba(239, 68, 68, 0.1);
	}
	.border-red-500-20 {
		border-color: rgba(239, 68, 68, 0.2);
	}
</style>
