import {SidebarType} from "$lib/home/sidebar/types";
import {writable} from "svelte/store";

export const sidebarShowingType = writable<null | SidebarType>(null);