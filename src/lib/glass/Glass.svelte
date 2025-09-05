<script lang="ts">
import {isAndroid} from "$lib/platform";

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
    glassEffectScale?: number;
    events?: any;
}

const componentId = crypto.randomUUID();
let props: Props = $props();
let glassEffectId = `glass-distortion-${componentId}`;
let { children, showShine = true, enableHoverAnimation = true, enableBlur = false, glassEffectScale = 0 } = props;
</script>

<div class="liquidGlass-wrapper {enableHoverAnimation ? 'hover-animation' : ''} {props.class}"
    style="
        --padding: {props.padding || '0.6rem'};
        --padding-hover: {props.paddingHover || '0.8rem'};
        {props.style}
    " {...props.events}>
    <div class="liquidGlass-effect {isAndroid() ? 'backdrop-blur-[8px]' : 'backdrop-blur-md'}
        {enableBlur ? 'liquidGlass-blur' : ''} {props.wrapperClass}"
        style="filter: url(#{glassEffectId}); {props.wrapperStyle}"></div>
    <div class="liquidGlass-tint {props.wrapperClass}" style="{props.wrapperStyle}"></div>
    {#if showShine}
        <div class="liquidGlass-shine {props.wrapperClass}" style="{props.wrapperStyle}"></div>
    {/if}
    <div class="liquidGlass-text w-full h-full {props.wrapperClass}" style="{props.wrapperStyle}">
        {@render children?.()}
    </div>
</div>

{#if glassEffectScale > 0 && !isAndroid()}
    <svg style="display: none">
        <filter
                id="{glassEffectId}"
                x="0%"
                y="0%"
                width="100%"
                height="100%"
                filterUnits="objectBoundingBox"
        >
            <feTurbulence
                    type="fractalNoise"
                    baseFrequency="0.01 0.01"
                    numOctaves="1"
                    seed="5"
                    result="turbulence"
            />
            <!-- Seeds: 14, 17,  -->

            <feComponentTransfer in="turbulence" result="mapped">
                <feFuncR type="gamma" amplitude="1" exponent="10" offset="0.5" />
                <feFuncG type="gamma" amplitude="0" exponent="1" offset="0" />
                <feFuncB type="gamma" amplitude="0" exponent="1" offset="0.5" />
            </feComponentTransfer>

            <feGaussianBlur in="turbulence" stdDeviation="3" result="softMap" />

            <feSpecularLighting
                    in="softMap"
                    surfaceScale="5"
                    specularConstant="1"
                    specularExponent="100"
                    lighting-color="white"
                    result="specLight"
            >
                <fePointLight x="-200" y="-200" z="300" />
            </feSpecularLighting>

            <feComposite
                    in="specLight"
                    operator="arithmetic"
                    k1="0"
                    k2="1"
                    k3="1"
                    k4="0"
                    result="litImage"
            />

            <feDisplacementMap
                    in="SourceGraphic"
                    in2="softMap"
                    scale={glassEffectScale}
                    xChannelSelector="R"
                    yChannelSelector="G"
            />
        </filter>
    </svg>
{/if}

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

  // .liquidGlass-blur {
  //   backdrop-filter: blur(var(--blur));
  //   -webkit-backdrop-filter: blur(var(--blur));
  // }

  .liquidGlass-effect {
    position: absolute;
    z-index: 0;
    inset: 0;

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