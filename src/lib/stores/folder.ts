import { writable } from "svelte/store";
import type { FolderData } from "$lib/home/music/types";

export const folderList = writable<FolderData[]>([]);
export const folderCurrent = writable<FolderData | null>(null);
