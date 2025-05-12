import { writable } from "svelte/store";

export let pageGotoRoute = writable<string | null>(null);
export let pageGotoShow = writable(true);
export let pageHomePlayerBarShow = writable(true);
