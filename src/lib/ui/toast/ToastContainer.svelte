<script lang="ts">
	import { isMobile } from '$lib/platform';
    import ToastService from '$lib/services/ToastService.svelte';
	import mobileStore from '$lib/stores/mobile.svelte';
	import Toast from './Toast.svelte';
	import { flip } from 'svelte/animate';
	import { fly } from 'svelte/transition';
</script>

<div
	class="pointer-events-none fixed right-0 top-0 z-[100000] flex h-screen w-full max-w-sm flex-col justify-start px-3"
    style="padding-top: {isMobile() ? mobileStore.statusBarHeight : 8}px;"
>
	{#each ToastService.toasts as toast (toast.id)}
		<div
			animate:flip={{ duration: 300 }}
			in:fly={{ x: 300, duration: 300 }}
			out:fly={{ x: 300, duration: 300 }}
            class="flex w-full justify-end"
		>
			<Toast {toast} />
		</div>
	{/each}
</div>
