<script lang="ts">
interface Props {
    children?: import("svelte").Snippet;
    class?: string;
    wrapperClass?: string;
    style?: string;
    wrapperStyle?: string;
    padding?: string;
    paddingHover?: string;
    showShine?: boolean;
    enableHoverAnimation?: boolean;
    // Note: Enable if needed, disabled by default to increase performance :)
    enableBlur?: boolean;
    events?: any;
}

let props: Props = $props();
let { children, showShine = true, enableHoverAnimation = true, enableBlur = false } = props;
</script>

<div class="liquidGlass-wrapper {enableHoverAnimation ? 'hover-animation' : ''} {props.class}"
    style="
        --padding: {props.padding || '0.6rem'};
        --padding-hover: {props.paddingHover || '0.8rem'};
        {props.style}
    " {...props.events}>
    <div class="liquidGlass-effect {enableBlur ? 'liquidGlass-blur' : ''} {props.wrapperClass}" style="{props.wrapperStyle}"></div>
    <div class="liquidGlass-tint {props.wrapperClass}" style="{props.wrapperStyle}"></div>
    {#if showShine}
        <div class="liquidGlass-shine {props.wrapperClass}" style="{props.wrapperStyle}"></div>
    {/if}
    <div class="liquidGlass-text w-full h-full {props.wrapperClass}" style="{props.wrapperStyle}">
        {@render children?.()}
    </div>
</div>

<style lang="scss">
  /* LIQUID GLASS STYLES */

  .liquidGlass-wrapper {
    position: relative;
    display: flex;
    overflow: hidden;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border-radius: 2rem;
    padding: var(--padding);

    color: black;

    box-shadow: 0 6px 6px rgba(0, 0, 0, 0.2), 0 0 20px rgba(0, 0, 0, 0.1);

    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 2.2);
  }

  .liquidGlass-blur {
    backdrop-filter: blur(12px);
  }

  .liquidGlass-effect {
    position: absolute;
    z-index: 0;
    inset: 0;

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

  .liquidGlass-wrapper,
  .liquidGlass-wrapper > div {
    border-radius: 2rem;
  }

  .hover-animation:hover {
    padding: var(--padding-hover);
    border-radius: 2.5rem;
  }

  .hover-animation:hover > div {
    border-radius: 2.5rem;
  }
</style>