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

export interface MusicPlayerInfo {
    currentPosition: number;
    isPlaying: boolean;
    music: MusicData | null;
}

export interface MusicPlayerSync {
    skip: number;
}
