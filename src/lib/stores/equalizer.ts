import {writable} from "svelte/store";

export const equalizerShow = writable(false);
export const equalizerValues = writable(Array(18).fill(0));