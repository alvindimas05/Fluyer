<script lang="ts">
import Button from "$lib/ui/components/Button.svelte";
import {useIntro} from "../viewmodels/useIntro.svelte";
import {isAndroid, isDesktop} from "$lib/platform";

const vm = useIntro();
</script>

<div
        class="h-full grid justify-items-center items-center animate__animated {vm.animatedClasses}"
        onanimationend={() => vm.onAnimationEnd(vm.animatedClasses)}
>
    <div class="grid justify-items-center text-center px-5 lg:px-0">
        <p class="font-bold text-2xl md:text-3xl">Let's Set Up Your Music</p>
        <p class="mt-3 text-lg md:text-xl">
            {#if isAndroid()}
                To get started, allow access and select your music folder.
            {:else if isDesktop()}
                Select your music folder to get started.
            {/if}
        </p>
        <Button class="rounded w-fit mt-5 p-2" onclick={vm.requestAction}>
            {#if isAndroid()}
                Allow Access & Choose Folder
            {:else if isDesktop()}
                Choose Folder
            {/if}
        </Button>
    </div>
</div>
