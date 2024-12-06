import { writable } from "svelte/store";

export let album = writable<null | string>(null);
export let spotifyAccessToken = writable<null | string>(null);
