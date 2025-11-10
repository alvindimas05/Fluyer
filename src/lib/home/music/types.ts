export interface AlbumData {
	name: string;
	artist: string;
	year: string;
	duration: string;
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
	bitsPerSample: number | null;
	sampleRate: number | null;
	genre: string | null;
	date: string | null;
}

export enum MusicSize {
	Music = 100,
	Album = 400,
	AnimatedBackground = 50,
}

export interface FolderData {
	path: string;
}

export interface MusicPlayerSync {
	index: number;
	currentPosition: number | null;
	isPlaying: boolean;
	duration: number;
}

export enum RepeatMode {
	None = "repeatNone",
	One = "repeatOne",
	All = "repeat",
}

export enum MusicListType {
	All = "all",
	Folder = "folder",
    Album = "album",
    Music = "music",
}
