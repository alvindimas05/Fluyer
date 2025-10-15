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
import CloseCircleOutline from "svelte-material-icons/CloseCircleOutline.svelte";
import Delete from "svelte-material-icons/Delete.svelte";
import Cog from "svelte-material-icons/Cog.svelte";
import Magnify from "svelte-material-icons/Magnify.svelte";
import PlaylistPlus from "svelte-material-icons/PlaylistPlus.svelte";
import Broom from "svelte-material-icons/Broom.svelte";
import Repeat from "svelte-material-icons/Repeat.svelte";
import RepeatOnce from "svelte-material-icons/RepeatOnce.svelte";
import Shuffle from "svelte-material-icons/Shuffle.svelte";
import Fullscreen from "svelte-material-icons/Fullscreen.svelte";
import DotsGrid from "svelte-material-icons/DotsGrid.svelte";
import Lock from "svelte-material-icons/Lock.svelte";
import FileDocumentOutline from "svelte-material-icons/FileDocumentOutline.svelte";
import TuneVerticalVariant from "svelte-material-icons/TuneVerticalVariant.svelte";
import Poll from "svelte-material-icons/Poll.svelte";
import Folder from "svelte-material-icons/Folder.svelte";
import MenuUp from "svelte-material-icons/MenuUp.svelte";
import MenuDown from "svelte-material-icons/MenuDown.svelte";

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
	[IconType.Remove]: CloseCircleOutline,
	[IconType.Trash]: Delete,
	[IconType.Settings]: Cog,
	[IconType.Search]: Magnify,
	[IconType.QueuePlaylist]: PlaylistPlus,
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
	[IconType.Equalizer]: TuneVerticalVariant,
	[IconType.Close]: CloseCircleOutline,
	[IconType.Visualizer]: Poll,
	[IconType.Folder]: Folder,
    [IconType.SortAsc]: MenuUp,
    [IconType.SortDesc]: MenuDown,
};

export default iconRegistryMaterial;
