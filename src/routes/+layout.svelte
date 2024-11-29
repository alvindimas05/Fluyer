<script lang="ts">
    import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
    import logHandler from "$lib/handlers/log";
    import "../app.scss";
    
    logHandler();
</script>

<div class="w-screen h-screen absolute overflow-x-hidden scrollbar-hidden">
    <slot/>
</div>
<AnimatedBackground/>