import { writable } from "svelte/store";

export let observerCounts = writable(0);
export let backgroundIsLight = writable(false);