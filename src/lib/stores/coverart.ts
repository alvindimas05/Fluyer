import type { CoverArtResponse } from "$lib/handlers/coverart";
import { writable } from "svelte/store";

export let coverArtCaches = writable<CoverArtResponse[]>([]);
