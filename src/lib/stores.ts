import { writable } from "svelte/store";

export let swipeMinimumTop = writable(0);
export let albumListScrollIndex = writable(-1);