import { IconType } from "$lib/ui/icon/types";
import ArrowUUpLeft from "phosphor-svelte/lib/ArrowUUpLeft";
import Broom from "phosphor-svelte/lib/Broom";
import CaretDown from "phosphor-svelte/lib/CaretDown";
import CaretUp from "phosphor-svelte/lib/CaretUp";
import ChartBar from "phosphor-svelte/lib/ChartBar";
import DotsSixVertical from "phosphor-svelte/lib/DotsSixVertical";
import Faders from "phosphor-svelte/lib/Faders";
import FileText from "phosphor-svelte/lib/FileText";
import Folder from "phosphor-svelte/lib/Folder";
import FrameCorners from "phosphor-svelte/lib/FrameCorners";
import Gear from "phosphor-svelte/lib/Gear";
import GridFour from "phosphor-svelte/lib/GridFour";
import LockSimple from "phosphor-svelte/lib/LockSimple";
import MagnifyingGlass from "phosphor-svelte/lib/MagnifyingGlass";
import MusicNote from "phosphor-svelte/lib/MusicNote";
import MusicNotes from "phosphor-svelte/lib/MusicNotes";
import PauseCircle from "phosphor-svelte/lib/PauseCircle";
import PlayCircle from "phosphor-svelte/lib/PlayCircle";
import QuestionMark from "phosphor-svelte/lib/QuestionMark";
import Queue from "phosphor-svelte/lib/Queue";
import Repeat from "phosphor-svelte/lib/Repeat";
import RepeatOnce from "phosphor-svelte/lib/RepeatOnce";
import Shuffle from "phosphor-svelte/lib/Shuffle";
import SkipBackCircle from "phosphor-svelte/lib/SkipBackCircle";
import SkipForwardCircle from "phosphor-svelte/lib/SkipForwardCircle";
import SpeakerHigh from "phosphor-svelte/lib/SpeakerHigh";
import SpeakerX from "phosphor-svelte/lib/SpeakerX";
import Trash from "phosphor-svelte/lib/Trash";
import VinylRecord from "phosphor-svelte/lib/VinylRecord";
import XCircle from "phosphor-svelte/lib/XCircle";

const iconRegistryPhospor = {
	[IconType.Unknown]: QuestionMark,
	[IconType.Play]: PlayCircle,
	[IconType.Pause]: PauseCircle,
	[IconType.Previous]: SkipBackCircle,
	[IconType.Next]: SkipForwardCircle,
	[IconType.Playing]: MusicNotes,
	[IconType.Note]: MusicNote,
	[IconType.PlayBack]: ArrowUUpLeft,
	[IconType.Back]: ArrowUUpLeft,
	[IconType.AlbumBack]: ArrowUUpLeft,
	[IconType.Speaker]: SpeakerHigh,
	[IconType.Mute]: SpeakerX,
	[IconType.Remove]: XCircle,
	[IconType.Trash]: Trash,
	[IconType.Settings]: Gear,
	[IconType.Search]: MagnifyingGlass,
	[IconType.QueueMusic]: Queue,
	[IconType.CleanQueue]: Broom,
	[IconType.RepeatNone]: Repeat,
	[IconType.RepeatPlayNone]: Repeat,
	[IconType.Repeat]: Repeat,
	[IconType.RepeatOne]: RepeatOnce,
	[IconType.Shuffle]: Shuffle,
	[IconType.Fullscreen]: FrameCorners,
	[IconType.DragOn]: DotsSixVertical,
	[IconType.DragOff]: LockSimple,
	[IconType.SaveLog]: FileText,
	[IconType.Equalizer]: Faders,
	[IconType.Close]: XCircle,
	[IconType.Visualizer]: ChartBar,
	[IconType.Folder]: Folder,
	[IconType.SortAsc]: CaretUp,
	[IconType.SortDesc]: CaretDown,
	[IconType.MusicListTypeAll]: GridFour,
	[IconType.MusicListTypeAlbum]: VinylRecord,
	[IconType.MusicListTypeMusic]: MusicNote,
	[IconType.MusicListTypeFolder]: Folder,
};

export default iconRegistryPhospor;
