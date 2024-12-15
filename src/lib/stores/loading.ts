import { writable } from "svelte/store";

export let loadingBackground = writable(false);
export let loadingMusicList = writable(false);
export let loadingShow = writable(false);
