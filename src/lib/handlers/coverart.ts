import MusicBrainzApi from "$lib/api/musicbrainz";
import { CommandsRoute } from "$lib/commands";
import type { MusicData } from "$lib/home/music/types";
import { invoke } from "@tauri-apps/api/core";

const mbApi = new MusicBrainzApi();
const CoverArt = {
    fromAlbum: async (album: string) => {
        try {
            let albumCover = await invoke(CommandsRoute.COVER_ART_FROM_ALBUM, { album });
            if(albumCover != null) return `data:image/png;base64,${albumCover}`;
            
            const url  = await mbApi.getAlbumImageFromAlbum(album);
            if(url == null) return null;
            
            let reqAlbum = await invoke(CommandsRoute.COVER_ART_REQUEST_ALBUM, { album, url });
            if(reqAlbum == null) return null;
            
            return `data:image/png;base64,${reqAlbum}`;
        } catch(err){
            console.error(err)
            return null;
        }
    },
    fromMusic: async (music: MusicData) => {
        if(music.album == null) return null;
        return CoverArt.fromAlbum(music.album);
    }
}

export default CoverArt;