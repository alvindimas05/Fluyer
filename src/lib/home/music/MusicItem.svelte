<script lang="ts">
    import type { MusicData } from "./types";

    export let music: MusicData;

    const albumArtist = music.album_artist && !music.artist.includes(music.album_artist) ? ` • ${music.album_artist}` : "";
</script>

<div class="grid grid-cols-[max-content_auto] py-2">
    <img class="w-16 rounded" src={`data:image/png;base64,${music.image}`} alt="Album">
    <div class="ms-3">
        <p class="font-medium">{music.title}</p>
        <p class="text-gray-400">{music.artist}{albumArtist}</p>
    </div>
</div>
