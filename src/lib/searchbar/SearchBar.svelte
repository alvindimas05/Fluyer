<script lang="ts">
    import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
    import type { MusicData } from "$lib/home/music/types";
    import { musicAlbumList, musicList } from "$lib/stores/music";
    
    let searchInput: HTMLInputElement;
    let searchValue = $state("");
    let showSearchBar = $state(false);
    
    function musicListFilterPredicate(music: MusicData){
        let search = searchValue.toLowerCase();
        return music.title?.toLowerCase().includes(search) || music.albumArtist?.toLowerCase()
            .includes(search);
    }
    
    function musicAlbumListFilterPredicate(album: MusicData[]){
        let search = searchValue.toLowerCase();
        return album[0].album?.toLowerCase().includes(search) || album[0].albumArtist?.toLowerCase()
            .includes(search)
    }
    
    
    async function addMusicListAndPlay(list: MusicData[]) {
    	await MusicController.addMusicList(MusicController.sortMusicList(list));
        
        const previousMusic = MusicController.currentMusic();
    	if (
    		previousMusic === null ||
    		(previousMusic !== null && MusicController.isCurrentMusicFinished())
    	)
    		MusicController.play();
    }
    
    
    function onKeyDown(
    	e: KeyboardEvent & {
    		currentTarget: EventTarget & Document;
    	},
    ) {
        if (e.ctrlKey){ showSearchBar = true;
            setTimeout(() => {
                searchInput.focus();
            }, 0);
        }
        if (e.key === "Escape") showSearchBar = false;
    }
</script>

<svelte:document onkeydown={onKeyDown}/>
{#if showSearchBar}
    <div
        class={`w-screen h-screen grid justify-items-center items-center fixed z-10 md:p-0`}
    >
        <div
            class="grid w-full md:w-[35rem] h-fit justify-items-center rounded border backdrop-blur-xl bg-gray-700 bg-opacity-40"
        >
            <div class="w-full px-4 py-3">
                <input class="w-full font-medium text-xl bg-transparent outline-none border-0 placeholder:text-gray-200" placeholder="Search"
                    bind:this={searchInput}
                    bind:value={searchValue}/>
            </div>
            {#if $musicList != null && $musicList != undefined && searchValue.length > 0}
                <div class="w-full border-t">
                    {#each $musicAlbumList.filter(musicAlbumListFilterPredicate).slice(0, 3) as album}
                        <button class="w-full grid grid-cols-[auto_1fr_auto] py-3 px-4 text-start hover:bg-gray-400 hover:bg-opacity-20"
                            onclick={() => addMusicListAndPlay(album)}>
                            <div class="pe-3">
                                <img class="w-8 rounded" 
                                    src={MusicController.getAlbumImageFromMusic(album[0]) ?? MusicConfig.defaultAlbumImage} alt="">
                            </div>
                            <p class="grid h-full items-center">{album[0].albumArtist ?? album[0].artist} - {album[0].album}</p>
                            <p class="grid h-full items-center">Album</p>
                        </button>
                    {/each}
                    {#each $musicList.filter(musicListFilterPredicate).slice(0, 5) as music}
                        <button class="w-full grid grid-cols-[auto_1fr_auto] py-3 px-4 text-start hover:bg-gray-400 hover:bg-opacity-20"
                            onclick={() => addMusicListAndPlay([music])}>
                            <div class="pe-3">
                                <img class="w-8 rounded" 
                                    src={MusicController.getAlbumImageFromMusic(music) ?? MusicConfig.defaultAlbumImage} alt="">
                            </div>
                            <p class="grid h-full items-center">{music.albumArtist ?? music.artist} - {music.title}</p>
                            <p class="grid h-full items-center">Music</p>
                        </button>
                    {/each}
                </div>
            {/if}
        </div>
    </div>
{/if}