<script lang="ts">
    import { run } from 'svelte/legacy';

    import type { MusicData } from "../music/types";
    import { musicList } from "$lib/stores/music";
    import AlbumItem from "./AlbumItem.svelte";
    
    let grouppedAlbums = $state(groupByAlbum());
    
    function groupByAlbum(): MusicData[][] {
        const albumsMap = $musicList.reduce((acc, item) => {
            if (item.album.trim() === '') {
                return acc;
            }
    
            if (!acc[item.album]) {
                acc[item.album] = [];
            }
    
            acc[item.album].push(item);
    
            return acc;
        }, {} as Record<string, MusicData[]>);
    
        return Object.values(albumsMap);
    }
    run(() => {
        $musicList, grouppedAlbums = groupByAlbum();
    });
</script>
<div class="grid auto-cols-[20rem] grid-rows-[1fr] w-full overflow-x-auto scrollbar-hidden">
    {#each Object.entries(grouppedAlbums) as [album, list]}
        <AlbumItem music={list[0]} />
    {/each}
</div>