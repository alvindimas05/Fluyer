import FolderController from "$lib/controllers/FolderController";
import MusicController from "$lib/controllers/MusicController";
import { MusicListType } from "$lib/home/music/types";
import musicStore from "$lib/stores/music.svelte";
import filterStore from "$lib/stores/filter.svelte";
import folderStore from "$lib/stores/folder.svelte";

export function useMusicList() {
    const rules = [
        [1280, 2.01, 4], [1024, 2.01, 3], [768, 2.01, 2],
        [1536, 1.01, 4], [1280, 1.01, 3], [768, 1.01, 2],
        [1536, 1.0, 4],  [1024, 1.0, 3],  [768, 1.0, 2],
    ];

    let columnCount = $state(1);

    function updateColumnCount() {
        const w = window.innerWidth;
        const dpi = window.devicePixelRatio;

        for (const [minW, minDppx, cols] of rules) {
            if (w >= minW && dpi >= minDppx) {
                columnCount = cols;
                return;
            }
        }
        columnCount = 1;
    }

    const updateSize = () => updateColumnCount();
    $effect(() => {
        musicStore.listType;
        updateSize();
    });


    const data = $derived.by(() => {
        if (!Array.isArray(musicStore.list)) return [];

        const search = filterStore.search.toLowerCase();
        const album = filterStore.album;
        const hasSearch = search.length > 0;
        const hasAlbum = !!album;
        const isFolderMode = musicStore.listType === MusicListType.Folder;

        const filteredMusic = MusicController.sortMusicList(
            musicStore.list.filter(music => {
                const matchesSearch = hasSearch &&
                    [
                        music.album, music.title,
                        music.artist, music.albumArtist
                    ].some(v => v?.toLowerCase().includes(search));

                const matchesAlbum = hasAlbum && album.name === music.album;
                const matchesFolder = FolderController.isMusicInFolder(
                    music,
                    folderStore.currentFolder
                );

                if (isFolderMode)
                    return matchesFolder && (!hasSearch || matchesSearch);

                if (!hasAlbum)
                    return !hasSearch || matchesSearch;

                return matchesAlbum && (!hasSearch || matchesSearch);
            })
        );

        let filteredFolders = folderStore.list.filter(f =>
            f.path.toLowerCase().includes(search)
        );

        if (!filterStore.bar.sortAsc) filteredFolders = [...filteredFolders].reverse();

        let finalList = filterStore.album
            ? MusicController.sortMusicList(filteredMusic)
            : [...filteredMusic];

        if (!filterStore.bar.sortAsc) finalList.reverse();

        const chunk = (arr: any[]) => {
            const rows = [];
            for (let i = 0; i < arr.length; i += columnCount)
                rows.push(arr.slice(i, i + columnCount));
            return rows;
        };

        const result = chunk(finalList);

        if (isFolderMode) {
            const nonEmpty = filteredFolders.filter(
                f => FolderController.getMusicListFromFolder(f).length > 0
            );
            result.push(...chunk(nonEmpty));
        }

        return result;
    });

    return {
        data,
        columnCount,
        updateSize,
    };
}
