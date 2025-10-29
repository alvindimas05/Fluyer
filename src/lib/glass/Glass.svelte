<script lang="ts">
import { isAndroid } from "$lib/platform";

interface Props {
    children?: import("svelte").Snippet;
    class?: string;
    style?: string;
    showShine?: boolean;
    shineColor?: string;
    showShadow?: boolean;
    enableBlur?: boolean;
    thisElement?: HTMLDivElement;
    events?: any;
}

let {
    children,
    shineColor = 'rgba(255, 255, 255, 0.5)',
    enableBlur = false,
    thisElement = $bindable<HTMLDivElement>(),
    ...props
}: Props = $props();

const getBlurClass = () => {
    if (!enableBlur) return '';
    return isAndroid() ? 'backdrop-blur-xs' : 'backdrop-blur-md';
};

const getHoverClasses = () => {
    return 'transition-all duration-[400ms] ease-[cubic-bezier(0.175,0.885,0.32,2.2)]';
};
</script>

<div
    class="{getBlurClass()} {getHoverClasses()}
        shadow-[inset_2px_2px_1px_0_var(--shine-color),inset_-1px_-1px_1px_1px_var(--shine-color),0_10px_15px_-3px_rgb(0_0_0_/_0.1),0_4px_6px_-4px_rgb(0_0_0_/_0.1)]
        {props.class ?? ''}"
    style="--shine-color: {shineColor}; {isAndroid() ? '-webkit-transform: translate3d(0, 0, 0);' : ''} {props.style || ''}"
    bind:this={thisElement}
    {...props.events}
>
    {@render children?.()}
</div>