import { IconType, IconThemeType } from "$lib/icon/types";
import type { Component } from "svelte";
import {
	ArrowCircleLeft, ArrowCounterClockwise, ArrowUUpLeft,
	Gear,
	MagnifyingGlass,
	MusicNote,
	MusicNotes,
	PauseCircle,
	PlayCircle, Queue,
	SkipBackCircle,
	SkipForwardCircle,
	SpeakerHigh,
	SpeakerX,
	Trash,
	XCircle,
} from "phosphor-svelte";

export const iconRegistry: Record<
	IconThemeType,
	Partial<Record<IconType, Component>>
> = {
	[IconThemeType.Phosphor]: {
		[IconType.Play]: PlayCircle,
		[IconType.Pause]: PauseCircle,
		[IconType.Previous]: SkipBackCircle,
		[IconType.Next]: SkipForwardCircle,
		[IconType.Playing]: MusicNotes,
		[IconType.Note]: MusicNote,
		[IconType.PlayBack]: ArrowUUpLeft,
		[IconType.AlbumBack]: ArrowCounterClockwise,
		[IconType.Speaker]: SpeakerHigh,
		[IconType.Mute]: SpeakerX,
		[IconType.Remove]: XCircle,
		[IconType.Trash]: Trash,
		[IconType.Settings]: Gear,
		[IconType.Search]: MagnifyingGlass,
		[IconType.QueuePlaylist]: Queue,
	},
	[IconThemeType.Lucide]: {},
	[IconThemeType.Material]: {},
};
