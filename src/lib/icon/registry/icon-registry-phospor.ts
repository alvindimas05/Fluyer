import {IconType} from "$lib/icon/types";
import {
    ArrowUUpLeft, Broom, DotsSixVertical, FileText, FrameCorners, Gear, LockSimple, MagnifyingGlass,
    MusicNote,
    MusicNotes,
    PauseCircle,
    PlayCircle,
    QuestionMark, Queue, Repeat, RepeatOnce, Shuffle,
    SkipBackCircle,
    SkipForwardCircle, SpeakerHigh, SpeakerX, Trash, XCircle
} from "phosphor-svelte";

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
    [IconType.QueuePlaylist]: Queue,
    [IconType.CleanPlaylist]: Broom,
    [IconType.RepeatNone]: Repeat,
    [IconType.RepeatPlayNone]: Repeat,
    [IconType.Repeat]: Repeat,
    [IconType.RepeatOne]: RepeatOnce,
    [IconType.Shuffle]: Shuffle,
    [IconType.Fullscreen]: FrameCorners,
    [IconType.DragOn]: DotsSixVertical,
    [IconType.DragOff]: LockSimple,
    [IconType.SaveLog]: FileText,
};

export default iconRegistryPhospor;