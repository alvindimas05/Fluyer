import { writable } from "svelte/store"
import type { MusicData } from "../music/types"

export let musicList = writable<MusicData[]>([]);