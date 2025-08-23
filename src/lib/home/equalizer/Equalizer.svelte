<script lang="ts">
    import {isAndroid, isMacos} from "$lib/platform";
import { equalizerShow, equalizerValues } from "$lib/stores/equalizer";
import MusicController from "$lib/controllers/MusicController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import UIController from "$lib/controllers/UIController";
import PersistentStoreController from "$lib/controllers/PersistentStoreController";
import Glass from "$lib/glass/Glass.svelte";
import { onMount } from "svelte";
const LABELS = [
	65, // 1b  - Sub-bass
	92, // 2b
	131, // 3b
	185, // 4b
	262, // 5b  - Bass
	370, // 6b
	523, // 7b
	740, // 8b
	1047, // 9b  - Midrange
	1480, // 10b
	2093, // 11b
	2960, // 12b
	4186, // 13b  - Upper mids
	5920, // 14b
	8372, // 15b
	11840, // 16b
	16744, // 17b
	20000, // 18b - Highs / air
];

let isMounted = $state(false);

function updateValues(index: number, value: number) {
	equalizerValues.update((values) => {
		values[index] = value;
		MusicController.setEqualizer(values);
		PersistentStoreController.equalizer.set(values);
		return values;
	});
}

onMount(() => {
    setTimeout(() => isMounted = true, 1000);
});
</script>

<!-- <div class="absolute top-0 left-0 w-full h-full z-10
    grid items-center justify-items-center"
    class:hidden={!$equalizerShow}> -->
<div class="absolute top-0 left-0 w-full h-full z-10
    grid items-center justify-items-center pointer-events-none
    {isMounted ? '' : 'invisible'}
    animate__animated {$equalizerShow ? 'animate__slideInUp' : 'animate__slideOutDown'}">
    <Glass class="w-[calc(100%-1.5rem)] md:w-fit h-[50vh] pointer-events-auto"
        wrapperClass="p-4 grid grid-rows-[min-content_auto]"
        enableBlur={true} enableHoverAnimation={!isMacos()}>
        <div class="w-full grid grid-cols-2">
            <div class="flex justify-start">
                <button class="w-8 my-2 ms-2"
                    onclick={() => UIController.toggleEqualizer(false)}><Icon type={IconType.Close} /></button>
            </div>
            <div class="flex justify-end">
                <Glass
                        class="w-fit text-white text-start px-3 my-2 !rounded-lg
                        hover:bg-white/10 cursor-pointer"
                        wrapperClass="!rounded-lg"
                        padding="0.5rem"
                        enableHoverAnimation={!isMacos()}
                        events={{
                            onclick: MusicController.resetEqualizer
                        }}
                >
                    <div>Reset</div>
                </Glass>
            </div>
        </div>
        <div class="w-full grid grid-cols-[repeat(18,1fr)] overflow-auto scrollbar-hidden">
            {#each LABELS as label, i}
                <div class="grid grid-rows-[min-content_min-content_auto_min-content_min-content]
                    text-center font-semibold">
                    <div>{label > 1000 ? (label / 1000).toFixed(1) : label}</div>
                    <div>{label > 1000 ? 'kHz' : 'Hz'}</div>
                    <input type="range" class="range-slider"
                       min={0}
                       max={20}
                       step={0.1}
                       value={$equalizerValues[i]}
                       onchange={(e) => updateValues(i, e.currentTarget.valueAsNumber)}/>
                    <div class="mt-2 min-w-[2.75rem]">{$equalizerValues[i]}</div>
                    <div>dB</div>
                </div>
            {/each}
        </div>
    </Glass>
</div>

<style lang="scss">
  .range-slider {
    @apply bg-transparent appearance-none pt-2;
    writing-mode: vertical-rl;

    &::-webkit-slider-thumb {
      @apply w-4 h-4 mr-[-4px]
        bg-white/90 rounded
        shadow-xl backdrop-blur-md text-sm cursor-pointer
        appearance-none outline-none;
    }

    &::-webkit-slider-runnable-track {
      @apply w-2 bg-white/40 rounded-full;
      height: 0.5rem;
    }
  }
</style>