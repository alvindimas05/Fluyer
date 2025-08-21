<script lang="ts">
interface Props {
    children?: import("svelte").Snippet;
    class?: string;
    style?: string;
    padding?: string;
    paddingHover?: string;
    showShine?: boolean;
}

let props: Props = $props();
let { children, showShine = true } = props;
</script>

<div class="liquidGlass-wrapper dock {props.class}"
    style="
        --padding: {props.padding || '0.6rem'};
        --padding-hover: {props.paddingHover || '0.8rem'};
        {props.style}
    ">
    <div class="liquidGlass-effect"></div>
    <div class="liquidGlass-tint"></div>
    {#if showShine}
        <div class="liquidGlass-shine"></div>
    {/if}
    <div class="liquidGlass-text w-full h-full">
        {@render children?.()}
    </div>
</div>

<style lang="scss">
  /* LIQUID GLASS STYLES */

  .liquidGlass-wrapper {
    position: relative;
    display: flex;
    overflow: hidden;

    color: black;

    box-shadow: 0 6px 6px rgba(0, 0, 0, 0.2), 0 0 20px rgba(0, 0, 0, 0.1);

    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 2.2);
  }

  .liquidGlass-effect {
    position: absolute;
    z-index: 0;
    inset: 0;

    backdrop-filter: blur(8px);
    filter: url(#glass-distortion);
    overflow: hidden;
    isolation: isolate;
  }

  .liquidGlass-tint {
    z-index: 1;
    position: absolute;
    inset: 0;
    //background: rgba(255, 255, 255, 0.25);
  }

  .liquidGlass-shine {
    position: absolute;
    inset: 0;
    z-index: 2;

    overflow: hidden;

    box-shadow: inset 2px 2px 1px 0 rgba(255, 255, 255, 0.5),
    inset -1px -1px 1px 1px rgba(255, 255, 255, 0.5);
  }

  .liquidGlass-text {
    z-index: 3;
    font-size: 2rem;
    color: black;
  }

  .dock {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border-radius: 2rem;
    padding: var(--padding);
  }

  .dock,
  .dock > div {
    border-radius: 2rem;
  }

  .dock:hover {
    padding: var(--padding-hover);
    border-radius: 2.5rem;
  }

  .dock:hover > div {
    border-radius: 2.5rem;
  }
</style>