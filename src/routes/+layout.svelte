<script lang="ts">
    import 'animate.css';
    import AnimatedBackground from "$lib/backgrounds/AnimatedBackground.svelte";
    import logHandler from "$lib/handlers/log";
    import "../app.scss";
    interface Props {
        children?: import('svelte').Snippet;
    }

    let { children }: Props = $props();
    
    logHandler();
</script>

<div class="w-screen h-screen absolute overflow-x-hidden scrollbar-hidden">
    {@render children?.()}
</div>
<AnimatedBackground/>