import { writable } from "svelte/store";
import type { AlbumData } from "$lib/home/music/types";

export const filterSearch = writable("");
export const filterAlbum = writable<null | AlbumData>(null);
