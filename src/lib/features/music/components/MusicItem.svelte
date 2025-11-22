<script lang="ts">
import { type MusicData, type FolderData } from "../types";
import { useMusicItem } from "../viewmodels/useMusicItem.svelte";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { isDesktop, isLinux } from "$lib/platform";

interface Props {
    music: MusicData;
    folder?: FolderData;
}

let { music, folder }: Props = $props();

const vm = useMusicItem(music, folder);
const shouldAnimate = isDesktop() && !isLinux();
</script>

<div class="relative text-sm md:text-base">
    <div class="grid grid-cols-[max-content_auto_max-content] py-2">
        {#await vm.albumImage}
            <div class="w-12 h-12 md:w-14 md:h-14 relative aspect-square"></div>
        {:then image}
            {#if image}
                <img
                        class="w-12 h-12 md:w-14 md:h-14 object-cover relative rounded {shouldAnimate && 'animate__animated animate__fadeIn'}"
                        src={image}
                        alt="Album"
                />
            {:else}
                <div class="w-12 h-12 md:w-14 md:h-14 relative aspect-square">
                    <Icon type={IconType.Folder} />
                </div>
            {/if}
        {/await}

        <div class="ms-3 overflow-hidden">
            <p class="font-medium text-sm/[14px] md:text-sm whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
                {vm.titleLabel}
            </p>
            <p class="text-opacity-background-90 whitespace-nowrap overflow-hidden text-xs/[14px] pt-[4px] md:text-xs md:pt-0 animate-scroll-overflow-text">
                {vm.mediumLabel}
            </p>
            <p class="text-xs/[14px] md:text-xs mt-[2px] text-opacity-background-90">
                {vm.smallLabel}
            </p>
        </div>

        <div class="w-12 h-12 md:w-14 md:h-14 ps-2"></div>
    </div>

    <div class="absolute top-0 left-0 py-2 w-full">
        <div class="w-full grid grid-cols-[max-content_auto_max-content] music-item-play">
            <button class="w-12 h-12 md:w-14 md:h-14" onclick={vm.addMusicAndPlay}>
                <div class="bg-black bg-opacity-40 grid box-border justify-items-center items-center rounded md:p-1">
                    <Icon type={IconType.Play}/>
                </div>
            </button>

            <div class="{folder ? 'cursor-pointer' : 'cursor-default'}" onclick={vm.selectFolder}></div>

            <div class="w-12 h-12 md:w-14 md:h-14 ps-4">
                <button class="w-full h-full aspect-square" onclick={vm.addMusic}>
                    <Icon type={IconType.QueuePlaylist} />
                </button>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
  .music-item-play {
    opacity: 0;

    &:hover {
      animation-name: fadeIn;
      animation-duration: 0.5s;
      animation-fill-mode: forwards;
    }
  }
</style>