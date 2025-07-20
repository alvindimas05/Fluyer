import { IconType } from "$lib/icon/types";

import HelpCircleOutline from "svelte-material-icons/HelpCircleOutline.svelte";
import PlayCircleOutline from "svelte-material-icons/PlayCircleOutline.svelte";
import PauseCircleOutline from "svelte-material-icons/PauseCircleOutline.svelte";
import SkipPreviousCircleOutline from "svelte-material-icons/SkipPreviousCircleOutline.svelte";
import SkipNextCircleOutline from "svelte-material-icons/SkipNextCircleOutline.svelte";
import MusicNote from "svelte-material-icons/MusicNote.svelte";
import ArrowULeftTop from "svelte-material-icons/ArrowULeftTop.svelte";
import VolumeHigh from "svelte-material-icons/VolumeHigh.svelte";
import VolumeOff from "svelte-material-icons/VolumeOff.svelte";
import Cancel from "svelte-material-icons/Cancel.svelte";
import Delete from "svelte-material-icons/Delete.svelte";
import Cog from "svelte-material-icons/Cog.svelte";
import Magnify from "svelte-material-icons/Magnify.svelte";
import PlaylistMusic from "svelte-material-icons/PlaylistMusic.svelte";
import Broom from "svelte-material-icons/Broom.svelte";
import Repeat from "svelte-material-icons/Repeat.svelte";
import RepeatOnce from "svelte-material-icons/RepeatOnce.svelte";
import Shuffle from "svelte-material-icons/Shuffle.svelte";
import Fullscreen from "svelte-material-icons/Fullscreen.svelte";
import DotsGrid from "svelte-material-icons/DotsGrid.svelte";
import Lock from "svelte-material-icons/Lock.svelte";
import FileDocumentOutline from "svelte-material-icons/FileDocumentOutline.svelte";

const iconRegistryMaterial = {
    [IconType.Unknown]: HelpCircleOutline,
    [IconType.Play]: PlayCircleOutline,
    [IconType.Pause]: PauseCircleOutline,
    [IconType.Previous]: SkipPreviousCircleOutline,
    [IconType.Next]: SkipNextCircleOutline,
    [IconType.Playing]: MusicNote,
    [IconType.Note]: MusicNote,
    [IconType.PlayBack]: ArrowULeftTop,
    [IconType.Back]: ArrowULeftTop,
    [IconType.AlbumBack]: ArrowULeftTop,
    [IconType.Speaker]: VolumeHigh,
    [IconType.Mute]: VolumeOff,
    [IconType.Remove]: Cancel,
    [IconType.Trash]: Delete,
    [IconType.Settings]: Cog,
    [IconType.Search]: Magnify,
    [IconType.QueuePlaylist]: PlaylistMusic,
    [IconType.CleanPlaylist]: Broom,
    [IconType.RepeatNone]: Repeat,
    [IconType.RepeatPlayNone]: Repeat,
    [IconType.Repeat]: Repeat,
    [IconType.RepeatOne]: RepeatOnce,
    [IconType.Shuffle]: Shuffle,
    [IconType.Fullscreen]: Fullscreen,
    [IconType.DragOn]: DotsGrid,
    [IconType.DragOff]: Lock,
    [IconType.SaveLog]: FileDocumentOutline,
};

export default iconRegistryMaterial;
