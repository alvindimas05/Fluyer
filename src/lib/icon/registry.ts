import IconPlayCircleRegular from "phosphor-icons-svelte/IconPlayCircleRegular.svelte";

import { IconType, IconThemeType } from "$lib/icon/types";
import type {Component} from "svelte";
import IconPauseCircleRegular from "phosphor-icons-svelte/IconPauseCircleRegular.svelte";
import IconMusicNotesRegular from "phosphor-icons-svelte/IconMusicNotesRegular.svelte";
import IconArrowUUpLeftRegular from "phosphor-icons-svelte/IconArrowUUpLeftRegular.svelte";
import IconSpeakerHighRegular from "phosphor-icons-svelte/IconSpeakerHighRegular.svelte";
import IconSpeakerXRegular from "phosphor-icons-svelte/IconSpeakerXRegular.svelte";
import IconSkipBackCircleRegular from "phosphor-icons-svelte/IconSkipBackCircleRegular.svelte";
import IconSkipForwardCircleRegular from "phosphor-icons-svelte/IconSkipForwardCircleRegular.svelte";
import IconXCircleRegular from "phosphor-icons-svelte/IconXCircleRegular.svelte";
import IconMusicNoteSimpleBold from "phosphor-icons-svelte/IconMusicNoteSimpleBold.svelte";

export const iconRegistry: Record<IconThemeType, Partial<Record<IconType, Component>>> = {
    [IconThemeType.Phosphor]: {
        [IconType.Play]: IconPlayCircleRegular,
        [IconType.Pause]: IconPauseCircleRegular,
        [IconType.Previous]: IconSkipBackCircleRegular,
        [IconType.Next]: IconSkipForwardCircleRegular,
        [IconType.Playing]: IconMusicNotesRegular,
        [IconType.Note]: IconMusicNoteSimpleBold,
        [IconType.Back]: IconArrowUUpLeftRegular,
        [IconType.Speaker]: IconSpeakerHighRegular,
        [IconType.Mute]: IconSpeakerXRegular,
        [IconType.Remove]: IconXCircleRegular,
    },
    [IconThemeType.Lucide]: {},
    [IconThemeType.Material]: {}
};
