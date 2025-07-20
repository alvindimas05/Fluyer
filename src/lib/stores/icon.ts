import {IconThemeType} from "$lib/icon/types";
import {writable} from "svelte/store";

export const iconTheme = writable<IconThemeType>(IconThemeType.Phosphor);