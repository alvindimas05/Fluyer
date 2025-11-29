<script lang="ts">
import Icon from "$lib/ui/icon/Icon.svelte";
import { IconType } from "$lib/ui/icon/types";
import View from "$lib/ui/components/View.svelte";
import musicStore from "$lib/stores/music.svelte";
import {MusicListType} from "$lib/features/music/types";
import folderStore from "$lib/stores/folder.svelte";
import useCollectionInfo from "$lib/features/collection/viewmodels/useCollectionInfo.svelte";

const vm = useCollectionInfo();
</script>
{#if vm.album || folderStore.currentFolder || musicStore.listType === MusicListType.Playlist}
    <View class="mx-3 mb-2 md:grid grid-cols-[auto_max-content] px-4 py-2 rounded-lg
        box-border animate__animated animate__fadeIn
        hover:px-5 hover:py-3">
        <div class="grid items-center">
            <div class="text-sm md:text-base font-medium overflow-hidden">
                <p class="whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text">{vm.label}</p>
            </div>
        </div>
        <div class="w-fit">
            <div>

            </div>
            {#await vm.showBackButton then showBackButton}
                <div class="grid gap-x-2 md:gap-x-3 mt-2 md:mt-0"
                     class:grid-cols-4={showBackButton}
                     class:grid-cols-3={!showBackButton}>
                    {#if showBackButton}
                        <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                                onclick={vm.handleBack}>
                            <Icon type={IconType.AlbumBack} />
                        </button>
                    {/if}
                    <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                            onclick={vm.addMusicListAndPlay}>
                        <Icon type={IconType.Play} />
                    </button>
                    <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                            onclick={vm.addMusicList}>
                        <Icon type={IconType.QueuePlaylist} />
                    </button>
                    <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center"
                            onclick={vm.playShuffle}>
                        <Icon type={IconType.Shuffle} />
                    </button>
                </div>
            {/await}
        </div>
    </View>
{:else}
    <div></div>
{/if}