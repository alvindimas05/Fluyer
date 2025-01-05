<script lang="ts">
    import type { MusicData } from "../music/types";
    import { musicList } from "$lib/stores/music";
    import AlbumItem from "./AlbumItem.svelte";
    import MusicController from "$lib/controllers/MusicController";

    let element: HTMLDivElement;
    let grouppedAlbums = $state(groupByAlbum());

    function groupByAlbum(): MusicData[][] {
        const albumsMap = MusicController.musicList()!.reduce(
            (acc, item) => {
                if (item.album === null || item.album.trim() === "") {
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

    function onMouseWheel(
        e: WheelEvent & {
            currentTarget: EventTarget & HTMLDivElement;
        },
    ) {
        e.preventDefault();
        element.scrollLeft += e.deltaY;
    }

    musicList.subscribe(() => (grouppedAlbums = groupByAlbum()));
</script>

<div
    class="grid auto-cols-[50%] md:auto-cols-[20%] tb:auto-cols-[20%] lg:auto-cols-[16.6667%] grid-rows-[1fr] w-full mt-2 overflow-x-auto scrollbar-hidden"
    bind:this={element}
    onwheel={onMouseWheel}
>
    {#each Object.entries(grouppedAlbums) as [album, musicList], index}
        <AlbumItem {musicList} {index} />
    {/each}
</div>
