import {
	type MusicData,
	MusicListType,
	RepeatMode,
} from "$lib/home/music/types";
import { writable } from "svelte/store";

export let musicList = writable<MusicData[] | null | undefined>(undefined);
export let musicIsPlaying = writable(false);
export let musicCurrentIndex = writable<number>(-1);
export let musicProgressValue = writable(0);
export let musicVolume = writable(1);
export let musicAlbumList = writable<MusicData[][]>([]);
export let musicPlaylist = writable<MusicData[]>([]);
export let musicRepeatMode = writable<RepeatMode>(RepeatMode.None);
export let musicReset = writable(false);
export let musicListType = writable(MusicListType.All);
export let musicPlaylistIds = writable<string[]>([]);
