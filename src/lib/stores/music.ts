import type { MusicData } from "$lib/home/music/types";
import { writable } from "svelte/store";

export let musicList = writable<MusicData[]>([]);
export let musicIsPlaying = writable(false);
export let musicPlayed = writable<MusicData | null>(null);
