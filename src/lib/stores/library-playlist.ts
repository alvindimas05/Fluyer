import {writable} from "svelte/store";
import type {LibraryPlaylistData} from "$lib/home/music/types";

export const libraryPlaylist = writable<LibraryPlaylistData[]>([]);