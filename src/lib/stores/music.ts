import type { MusicData } from "$lib/home/music/types";
import { writable } from "svelte/store";

export let musicList = writable<MusicData[] | null | undefined>(undefined);
export let musicIsPlaying = writable(false);
export let musicCurrentIndex = writable<number>(-1);
export let musicProgressValue = writable(0);
export let musicVolume = writable(1);
export let musicProgressIntervalId = writable<null | ReturnType<
    typeof setInterval
>>(null);
export let musicAlbumList = writable<MusicData[][]>([]);
export let musicPlaylist = writable<MusicData[]>([]);
