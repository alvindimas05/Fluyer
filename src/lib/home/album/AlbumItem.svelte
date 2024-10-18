<script lang="ts">
    import type { MusicData } from "../music/types";

    export let music: MusicData;
</script>

<div class="px-3 py-6 text-white row-[1] col-auto">
    <img class="rounded-lg" src={`data:image/png;base64,${music.image}`} alt="Album">
    <p class="font-medium text-xl mt-2">{music.album}</p>
    <p class="text-lg text-gray-300">{music.album_artist ?? music.artist}</p>
</div>