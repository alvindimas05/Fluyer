import { writable } from "svelte/store"

export let album = writable('/test-album.jpg')
export let spotifyAccessToken = writable<null | string>(null);
export let musicValue = writable(0);