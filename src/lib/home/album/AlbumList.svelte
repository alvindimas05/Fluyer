<script lang="ts">
    import type { MusicData } from "../music/types";
    import { musicList } from "$lib/stores/music";
    import AlbumItem from "./AlbumItem.svelte";
    import MusicController from "$lib/MusicController";

    let grouppedAlbums = $state(groupByAlbum());

    function groupByAlbum(): MusicData[][] {
        const albumsMap = MusicController.musicList().reduce(
            (acc, item) => {
                if (item.album.trim() === "") {
                    return acc;
                }

                if (!acc[item.album]) {
                    acc[item.album] = [];
                }

                acc[item.album].push(item);

                return acc;
            },
            {} as Record<string, MusicData[]>,
        );

        return Object.values(albumsMap);
    }
    musicList.subscribe(() => (grouppedAlbums = groupByAlbum()));
</script>

<div
    class="grid auto-cols-[20rem] grid-rows-[1fr] w-full mt-2 overflow-x-auto scrollbar-hidden"
>
    {#each Object.entries(grouppedAlbums) as [album, list], index}
        <AlbumItem music={list[0]} {index} />
    {/each}
</div>
