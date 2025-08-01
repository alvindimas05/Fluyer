<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { filterAlbum } from "$lib/stores/filter";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import FilterController from "$lib/controllers/FilterController";

let album = $derived($filterAlbum);

function handleBack() {
	FilterController.setFilterAlbum(null);
}

async function addMusicListAndPlay() {
	await MusicController.resetAndAddMusicList(
		MusicController.sortMusicList(album!.musicList),
	);
	MusicController.play();
}

async function addMusicList() {
	MusicController.addMusicList(MusicController.sortMusicList(album!.musicList));
}

async function playShuffle() {
	MusicController.playShuffle(album!.musicList);
}
</script>
{#if album}
    <div class="px-3 pb-2 animate__animated animate__fadeIn">
        <div class="w-full md:grid grid-cols-[auto_max-content] px-4 py-2 bg-white/15 rounded-lg shadow-md">
            <div class="grid items-center">
                <div class="text-sm md:text-base font-medium text-white overflow-hidden">
                    <p class="whitespace-nowrap overflow-x-hidden animate-scroll-overflow-text">
                        {[album.name, album.artist, album.year, album.duration]
                            .filter(Boolean).join(` ${MusicConfig.separator} `)}
                    </p>
                </div>
            </div>
            <div class="w-fit grid grid-cols-4 gap-x-2 md:gap-x-3 mt-2 md:mt-0">
                <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center text-white"
                        onclick={handleBack}>
                    <Icon type={IconType.AlbumBack} />
                </button>
                <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center text-white"
                        onclick={addMusicListAndPlay}>
                    <Icon type={IconType.Play} />
                </button>
                <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center text-white"
                        onclick={addMusicList}>
                    <Icon type={IconType.QueuePlaylist} />
                </button>
                <button class="w-6 h-6 md:w-7 md:h-7 flex items-center justify-center text-white"
                        onclick={playShuffle}>
                    <Icon type={IconType.Shuffle} />
                </button>
            </div>
        </div>
    </div>
{:else}
    <div></div>
{/if}