<script lang="ts">
import {musicList, musicListType} from "$lib/stores/music";
import MusicItem from "./MusicItem.svelte";
import {VList} from "virtua/svelte";
import {onDestroy, onMount} from "svelte";
import {filterAlbum, filterSearch} from "$lib/stores/filter";
import MusicController from "$lib/controllers/MusicController";
import {folderCurrent, folderList} from "$lib/stores/folder";
import {MusicListType} from "$lib/home/music/types";
import FolderController from "$lib/controllers/FolderController";
import {playerBarHeight} from "$lib/stores/playerbar";
import {filterBarSortAsc} from "$lib/stores/filterbar";

// Responsive rules: [minWidth, maxDppxExclusive, columns]
const rules = [
	[1280, 2.01, 4], // xhdpi
	[1024, 2.01, 3],
	[768, 2.01, 2],

	[1536, 1.01, 4], // hdpi
	[1280, 1.01, 3],
	[768, 1.01, 2],

	[1536, 1.0, 4], // default
	[1024, 1.0, 3],
	[768, 1.0, 2],
];

let columnCount = $state(1);
function updateColumnCount() {
	// if(type === "folder") {
	// 	columnCount = 1;
	// 	return;
	// }

	const w = window.innerWidth;
	const dpi = window.devicePixelRatio;

	for (const [minW, minDppx, cols] of rules) {
		if (w >= minW && dpi >= minDppx) {
			columnCount = cols;
			return;
		}
	}
	columnCount = 1;
}

function updateSize() {
	updateColumnCount();
}

let data = $derived.by(() => {
	if (!Array.isArray($musicList)) return [];
	let list = MusicController.sortMusicList($musicList.filter((music) => {
		const search = $filterSearch.toLowerCase();
		const album = $filterAlbum;

		const hasSearch = !!search;
		const hasAlbum = !!album;

		const matchesSearch =
			hasSearch &&
			(music.album?.toLowerCase().includes(search) ||
				music.title?.toLowerCase().includes(search) ||
				music.artist?.toLowerCase().includes(search) ||
				music.albumArtist?.toLowerCase().includes(search));

		const matchesAlbum = hasAlbum && album.name === music.album;

		const matchesFolder = FolderController.isMusicInFolder(music, $folderCurrent);

		if($musicListType === MusicListType.Folder) {
			return matchesFolder && (!hasSearch || matchesSearch);
		} else if (!hasAlbum) {
			return !hasSearch || matchesSearch;
		} else {
			return matchesAlbum && (!hasSearch || matchesSearch);
		}
	}));
    if(!$filterBarSortAsc) list.reverse();

	let _folderList = $folderList.filter((folder) => {
		const search = $filterSearch.toLowerCase();
		return folder.path.toLowerCase().includes(search);
	});
    if(!$filterBarSortAsc) _folderList.reverse();

	if ($filterAlbum) list = MusicController.sortMusicList(list);

	const result: any[][] = [];
	for (let i = 0; i < list.length; i += columnCount) {
		result.push(list.slice(i, i + columnCount));
	}
	if ($musicListType === MusicListType.Folder){
		_folderList = _folderList.filter((folder) => FolderController.getMusicListFromFolder(folder).length > 0);

		for (let i = 0; i < _folderList.length; i += columnCount) {
			result.push(_folderList.slice(i, i + columnCount));
		}
	}
	return result;
});

let unsubscribeMusicListType = musicListType.subscribe(() => setTimeout(updateSize));
onMount(() => {
	updateSize();
});

onDestroy(() => {
	unsubscribeMusicListType();
});
</script>

<svelte:window onresize={updateSize} />
<div class="h-full px-3 text-white">
	{#if data && columnCount}
		<VList class="scrollbar-hidden" {data}
		   style="padding-bottom: {$playerBarHeight}px;"
		   getKey={(_, i) => i}>
			{#snippet children(list)}
				<div class="grid gap-x-6" style="grid-template-columns: repeat({columnCount}, minmax(0, 1fr))">
					{#each list as data}
						{#if 'duration' in data}
							<MusicItem music={data} />
						{:else}
							<MusicItem folder={data} />
						{/if}
					{/each}
				</div>
			{/snippet}
		</VList>
	{/if}
</div>
