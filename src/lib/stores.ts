import { writable } from "svelte/store";

export let spotifyAccessToken = writable<null | string>(null);
export let swipeMinimumTop = writable(0);
