import filterStore from "$lib/stores/filter.svelte";
import MetadataService from "$lib/services/MetadataService.svelte";
import musicStore from "$lib/stores/music.svelte";
import {type AlbumData, type MusicData, MusicListType} from "$lib/features/music/types";
import ProgressService from "$lib/services/ProgressService.svelte";

export function useAlbumItem(musicList: MusicData[], index: number){
    let music = $derived(musicList[0]);

    let isValidFilterAlbum = $derived(
        filterStore.album && music.album && filterStore.album.name === music.album,
    );

    let albumImage = $derived.by(() => MetadataService.getMusicCoverArt(music));

    async function setFilterAlbum() {
        const isAlbumType = musicStore.listType === MusicListType.Album;
        musicStore.listType = MusicListType.All;
        filterStore.album = {
            name: music.album,
            artist: music.albumArtist ?? music.artist,
            year: MetadataService.getYearFromDate(music.date),
            duration: ProgressService.formatDuration(
                musicList.map((m) => m.duration).reduce((a, b) => a + b, 0),
            ),
            musicList,
        } as AlbumData;
        if(isAlbumType) setTimeout(() => musicStore.albumListScrollIndex = index, 500);
    }

    return {
        get isValidFilterAlbum() {
            return isValidFilterAlbum;
        },
        get albumImage() {
            return albumImage;
        },
        get music() {
            return music;
        },
        setFilterAlbum,
    };
}