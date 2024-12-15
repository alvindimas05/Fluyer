import type { MusicData } from "$lib/home/music/types";
import { writable } from "svelte/store";

export let musicList = writable<MusicData[] | null | undefined>(undefined);
export let musicIsPlaying = writable(false);
export let musicCurrent = writable<MusicData | null>(null);
export let musicProgressValue = writable(0);
export let musicProgressIntervalId = writable<null | ReturnType<
	typeof setInterval
>>(null);
export let musicsNext = writable<MusicData[]>([]);
