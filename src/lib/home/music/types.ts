export interface MusicData {
	path: string;
	filename: string;
	duration: number;

	title: string | null;
	artist: string | null;
	album: string | null;
	album_artist: string | null;
	track_number: string | null;
	image: string | null;
}
