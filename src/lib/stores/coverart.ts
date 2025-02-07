import type { CoverArtResponse } from "$lib/handlers/coverart";
import { writable } from "svelte/store";

export let coverArtAlbumCaches = writable<CoverArtResponse[]>([]);
