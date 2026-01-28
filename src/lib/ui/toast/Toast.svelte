<script lang="ts">
	import { ToastType, type IToast } from '$lib/services/ToastService.svelte';
	import ToastService from '$lib/services/ToastService.svelte';

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
<div
	class="pointer-events-auto relative mb-3 flex w-full max-w-sm items-center gap-3 overflow-hidden rounded border px-3 py-2 shadow-xl transition-all"
	class:bg-gray-500-40={toast.toastType === ToastType.Info}
	class:border-white-20={toast.toastType === ToastType.Info}
	class:text-white={toast.toastType === ToastType.Info}
	class:bg-red-500-10={toast.toastType === ToastType.Error}
	class:border-red-500-20={toast.toastType === ToastType.Error}
	class:text-red-100={toast.toastType === ToastType.Error}
    style:backdrop-filter="blur(4px)"
    style:-webkit-backdrop-filter="blur(4px)"
	role="alert"
    onclick={close}
>
    <div class="shrink-0">
        {#if toast.toastType === ToastType.Info}
             <!-- Currently no specific icon for info in original toast, but good to have. Using a generic one or none? 
                  Original didn't use an icon explicitly in the CSS, just text. I'll stick to text for now unless I find icons. -->
             <!-- Assuming we might want an icon, but let's keep it simple first matching original -->
        {:else if toast.toastType === ToastType.Error}
             <!-- IconType.Error might exist, let's check or just skip icon if not sure -->
        {/if}
    </div>

	<div class="flex-1">
		{toast.message}
	</div>
</div>

<style>
    /* Tailwind utilities used in classes, custom colors handled via utility classes or inline styles if specific opacity needed */
    .bg-gray-500-40 {
        @apply bg-gray-500/40;
    }
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
