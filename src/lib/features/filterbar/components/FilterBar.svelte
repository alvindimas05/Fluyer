<script lang="ts">
import Icon from "$lib/ui/icon/Icon.svelte";
import {IconType} from "$lib/ui/icon/types";
import {isMacos, isMobile} from "$lib/platform";
import Toggle from "$lib/ui/components/Toggle.svelte";
import Button from "$lib/ui/components/Button.svelte";
import Input from "$lib/ui/components/Input.svelte";
import filterStore from "$lib/stores/filter.svelte";
import mobileStore from "$lib/stores/mobile.svelte";
import musicStore from "$lib/stores/music.svelte";
import {useFilterBar} from "../viewmodels/useFilterBar.svelte";

const vm = useFilterBar();
</script>

<svelte:window onresize={vm.updateSize} />
<div class="w-full grid gap-y-2 px-3 sm:px-0 sm:pb-3 pointer-events-none
        {isMacos() ? 'sm:justify-end' : ''}
        {isMacos() ? 'right-0' : 'left-0'}
        animate__animated animate__fadeIn animate__slow"
    style="margin-top: {isMobile() ? mobileStore.statusBarHeight : 8}px;
        grid-template-columns: {vm.state.gridSize};"
    bind:this={vm.element}>
    <div class="h-fit sm:h-auto grid pointer-events-none
        grid-cols-[min-content_1fr]
        sm:mx-3 gap-x-2 sm:gap-x-4
        {isMacos() ? 'justify-end' : 'justify-start'}">
        <div>
            <Button class="h-full aspect-square rounded grid justify-center pointer-events-auto p-[3.5px] sm:p-0"
                    onclick={vm.toggleSort}>
                <div class="w-5">
                    {#if filterStore.bar.sortAsc}
                        <Icon type={IconType.SortAsc} />
                    {:else}
                        <Icon type={IconType.SortDesc} />
                    {/if}
                </div>
            </Button>
        </div>
        <Toggle class="w-full h-full pointer-events-auto"
                iconStyle="width: {vm.iconSize}px;"
                options={vm.musicListOptions}
                selected={musicStore.listType}
                onchange={vm.handleToggleChange}
        />
    </div>
    <Input
            class="h-fit sm:h-full pointer-events-auto p-0 sm:mx-3
            {isMacos() ? 'order-first sm:order-last' : 'order-first'} rounded"
            icon={IconType.Search}
            placeholder="Search..."
            bind:value={filterStore.search}
    />
</div>