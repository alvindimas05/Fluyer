export interface AlbumData {
	name: string;
	artist: string;
	musicList: MusicData[];
}

export interface MusicData {
	path: string;
	filename: string;
	duration: number;

	title: string | null;
	artist: string | null;
	album: string | null;
	albumArtist: string | null;
	trackNumber: string | null;
	image: string | null;
}

export interface MusicPlayerSync {
	index: number;
	currentPosition: number;
	isPlaying: boolean;
	duration: number;
}

export enum RepeatMode {
	None = "repeatNone",
	One = "repeatOne",
	All = "repeat",
}
