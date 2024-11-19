<script lang="ts">
    import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import "../app.scss";
    
    window.addEventListener('error', e => invoke('log_error', { message: e.error }));
    window.addEventListener('unhandledrejection', e => invoke('log_error', { message: e.reason }));
</script>

<div class="w-screen h-screen absolute overflow-x-hidden scrollbar-hidden">
    <slot/>
</div>
<AnimatedBackground/>