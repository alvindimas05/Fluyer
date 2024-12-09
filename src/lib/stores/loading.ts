import { writable } from "svelte/store";

export let loadingMusicList = writable(false);
export let loadingAll = writable(false);
export let loadingShow = writable(false);
