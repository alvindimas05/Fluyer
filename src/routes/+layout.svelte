<script lang="ts">
    import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
    import "../app.scss";
</script>

<div class="w-screen h-screen absolute overflow-x-hidden scrollbar-hidden">
    <slot/>
</div>
<AnimatedBackground/>