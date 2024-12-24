<script lang="ts">
import type { MusicData } from "../music/types";
import { musicList } from "$lib/stores/music";
import AlbumItem from "./AlbumItem.svelte";
import MusicController from "$lib/controllers/MusicController";

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
musicList.subscribe(() => (grouppedAlbums = groupByAlbum()));
</script>

<!-- FIXME: Mouse scroll horizontal not working -->
<div
class="grid auto-cols-[50%] md:auto-cols-[25%] tb:auto-cols-[20%] lg:auto-cols-[16.6667%] grid-rows-[1fr] w-full mt-2 overflow-x-auto scrollbar-hidden"
>
    {#each Object.entries(grouppedAlbums) as [album, musicList], index}
        <AlbumItem {musicList} {index} />
    {/each}
</div>
