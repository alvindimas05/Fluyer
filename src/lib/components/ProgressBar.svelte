<script lang="ts">
interface Props {
    value: number;
    min?: number;
    max?: number;
    step?: number;
    progressPercentage: number;
    showTooltip?: boolean;
    tooltipFormatter?: (percentage: number) => string;
    class?: string;
    onValueChange?: (value: number) => void;
    onProgressClick?: (percentage: number) => void;
    onProgressEnter?: (percentage: number) => void;
    onProgressMove?: (percentage: number) => void;
    onProgressLeave?: (percentage: number) => void;
    size?: "sm" | "md" | "lg";
}

let {
    value = $bindable(),
    min = 0,
    max = 100,
    step = 1,
    progressPercentage,
    showTooltip = false,
    tooltipFormatter = (percentage: number) => `${percentage.toFixed(0)}%`,
    class: className = "",
    onValueChange,
    onProgressClick,
    onProgressEnter,
    onProgressMove,
    onProgressLeave,
    size = "md"
}: Props = $props();

const tooltipMargin = 12;

let tooltip: HTMLDivElement;
let tooltipPosition = $state(0);
let tooltipVisible = $state(false);
let tooltipText = $state("0:00");
let touchLastX = $state(0);
let containerWidth = $state(0);
let container: HTMLDivElement;

function setTooltipVisible(visible: boolean, percentage?: number) {
    if (!showTooltip) return;
    if (visible && percentage !== undefined) {
        tooltipText = tooltipFormatter(percentage);
    }
    tooltipVisible = visible;
}

function getPointerPercentage(x: number) {
    if(tooltip){
        tooltipPosition = x - tooltip.offsetWidth / 2;

        if (tooltipPosition < tooltipMargin) tooltipPosition = tooltipMargin;
        else if (tooltipPosition + tooltip.offsetWidth > containerWidth - tooltipMargin)
            tooltipPosition = containerWidth - tooltip.offsetWidth - tooltipMargin;
    }

    return (x / containerWidth) * 100;
}

function getEventPercentage(e: MouseEvent | TouchEvent) {
    updateContainerWidth();

    const rect = container.getBoundingClientRect();
    const x = 'touches' in e ? e.touches[0].clientX - rect.left : e.clientX - rect.left;

    if ('touches' in e) touchLastX = x;

    return getPointerPercentage(x);
}

// Shared event handlers
function handleEnter(e: MouseEvent | TouchEvent) {
    const percentage = getEventPercentage(e);
    onProgressEnter?.(percentage);
    setTooltipVisible(true, percentage);
}

function handleMove(e: MouseEvent | TouchEvent) {
    const percentage = getEventPercentage(e);
    onProgressMove?.(percentage);
    setTooltipVisible(true, percentage);
}

function handleLeave(e: MouseEvent | TouchEvent) {
    const percentage = getEventPercentage(e);
    onProgressLeave?.(percentage);
    setTooltipVisible(false);
}

// Mouse and touch wrappers
const handleMouseEnter = (e: MouseEvent) => handleEnter(e);
const handleMouseMove = (e: MouseEvent) => handleMove(e);
const handleMouseLeave = (e: MouseEvent) => handleLeave(e);
const handleTouchStart = (e: TouchEvent) => handleEnter(e);
const handleTouchMove = (e: TouchEvent) => handleMove(e);
const handleTouchEnd = () => {
    const percentage = (touchLastX / containerWidth) * 100;
    onProgressLeave?.(percentage);
    onProgressClick?.(percentage);
    setTooltipVisible(false);
};

function handleClick(e: MouseEvent) {
    const percentage = getEventPercentage(e);
    onProgressClick?.(percentage);
}

function updateContainerWidth() {
    if (container) {
        containerWidth = container.offsetWidth;
    }
}

function getProgressHeight(){
    switch (size){
        case "sm":
            return 3;
        case "md":
            return 4;
        case "lg":
            return 5;
    }
}

function getHandlerHeight(){
    switch (size){
        case "sm":
            return 16;
        case "md":
            return 28;
        case "lg":
            return 36;
    }
}

$effect(() => {
    updateContainerWidth();
});
</script>

<svelte:window onresize={updateContainerWidth} />

<div class="relative {className}" bind:this={container}>
    {#if showTooltip}
        <div
                class="w-fit absolute top-[-2.5rem] border rounded-lg px-2 py-1 shadow-xl text-sm backdrop-blur-xl"
                style="
				left: {tooltipPosition}px;
				visibility: {tooltipVisible ? 'visible' : 'hidden'};
			"
                bind:this={tooltip}
        >
            {tooltipText}
        </div>
    {/if}

    <div class="absolute w-full left-0 cursor-pointer z-10"
         style="
            bottom: -{getHandlerHeight() / 2}px;
            height: {getHandlerHeight()}px;
         "
         onmouseenter={handleMouseEnter}
         onmousemove={handleMouseMove}
         onmouseleave={handleMouseLeave}
         ontouchstart={handleTouchStart}
         ontouchmove={handleTouchMove}
         ontouchend={handleTouchEnd}
         onclick={handleClick}
    ></div>

    <input
            class="w-full absolute backdrop-blur-lg progress-bar"
            type="range"
            style="
                --progress-height: {getProgressHeight()}px;
                --progress-width: {progressPercentage}%;
            "
            bind:value
            {min}
            {max}
            {step}
            onchange={() => onValueChange?.(value)}
    />
    <input
            class="w-full absolute progress-bar-end"
            type="range"
            style="
                --progress-height: {getProgressHeight()}px;
                --progress-width: {progressPercentage}%;
            "
            bind:value
            {min}
            {max}
            {step}
    />
</div>

<style lang="scss">
.progress-bar-track {
    background: linear-gradient(to right, white var(--progress-width), transparent var(--progress-width));
    &-end {
        background: linear-gradient(to right, white var(--progress-width), white var(--progress-width));
    }
}

.progress-bar {
    @apply cursor-pointer outline-0;
    appearance: none;
    -webkit-appearance: none;
    background: transparent;
    transition: linear 0.2s;

    &::-webkit-slider-runnable-track {
        height: var(--progress-height);
        @apply rounded-full;
        @extend .progress-bar-track;
    }

    &::-webkit-slider-thumb {
        @apply invisible;
    }

    &-end {
        @extend .progress-bar;
        opacity: .3;
        height: var(--progress-height);

        &::-webkit-slider-runnable-track {
            @apply rounded-full;
            @extend .progress-bar-track-end;
        }
    }
}
</style>