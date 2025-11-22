<script lang="ts">
import {
    type FolderData,
    type MusicData,
    MusicListType,
} from "./types";
import MusicController, { MusicConfig } from "$lib/controllers/MusicController";
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { isDesktop, isLinux } from "$lib/platform";
import FolderController from "$lib/controllers/FolderController";
import { musicListType } from "$lib/stores/music.svelte";
import { folderCurrent } from "$lib/stores/folder";
import ToastController from "$lib/controllers/ToastController";

interface Props {
    music: MusicData;
    folder?: FolderData;
}

let { music, folder }: Props = $props();

let albumImage = $derived.by(async () => {
    return folder
        ? await FolderController.getImageFromPath(folder.path)
        : await MusicController.getAlbumImageFromMusic(music);
});

let titleLabel = $derived.by(() => {
    if (folder) {
        return $folderCurrent
            ? folder.path.split(FolderController.pathSeparator).pop()
            : folder.path;
    }
    return $musicListType === MusicListType.Folder ? music.filename : music.title;
});

let mediumLabel = $derived.by(() => {
    if (folder) return "Folder";

    const album = music.album ? `${music.album} ${MusicConfig.separatorAlbum} ` : "";
    const artist = music.artist ?? MusicConfig.defaultArtist;
    return `${album}${artist}`;
});

let smallLabel = $derived.by(() => {
    if (folder) {
        const folderMusic = FolderController.getMusicListFromFolder(folder);
        const totalDuration = folderMusic.reduce((acc, m) => acc + m.duration, 0);
        const durationText = MusicController.parseMilisecondsIntoText(totalDuration);
        return `${folderMusic.length} ${MusicConfig.separator} ${durationText}`;
    }

    const duration = MusicController.parseMilisecondsIntoText(music.duration);
    const resolution = [
        music.bitsPerSample && `${music.bitsPerSample}-bit`,
        MusicController.parseSampleRateIntoText(music.sampleRate),
    ].filter(Boolean);

    if (!resolution.length) return duration;

    const audioResolution = resolution.join(MusicConfig.separatorAudio);
    return `${audioResolution} ${MusicConfig.separator} ${duration}`;
});

async function addMusicAndPlay() {
    if (music) {
        await MusicController.resetAndAddMusic(music);
    } else {
        await MusicController.resetAndAddMusicList(
            FolderController.getMusicListFromFolder(folder!)
        );
    }
    if (!MusicController.isPlaying) MusicController.play();
}

async function addMusic() {
    const musicList = music
        ? [music]
        : FolderController.getMusicListFromFolder(folder!);

    if (music) {
        await MusicController.addMusic(music);
    } else {
        await MusicController.resetAndAddMusicList(musicList);
    }

    const title = music.title ?? music.filename ?? MusicConfig.defaultTitle;
    const artist = music.artist ?? MusicConfig.defaultArtist;
    ToastController.info(
        `Added music to queue: ${title} ${MusicConfig.separatorAlbum} ${artist}`
    );
}

async function selectFolder() {
    if (folder) await FolderController.setFolder(folder);
}

const shouldAnimate = isDesktop() && !isLinux();
</script>

<div class="relative text-sm md:text-base">
    <div class="grid grid-cols-[max-content_auto_max-content] py-2">
        {#await albumImage}
            <div class="w-12 h-12 md:w-14 md:h-14 relative aspect-square"></div>
        {:then image}
            {#if image}
                <img
                        class="w-12 h-12 md:w-14 md:h-14 object-cover relative rounded {shouldAnimate && 'animate__animated animate__fadeIn'}"
                        src={image}
                        alt="Album"
                />
            {:else}
                <div class="w-12 h-12 md:w-14 md:h-14 relative aspect-square">
                    <Icon type={IconType.Folder} />
                </div>
            {/if}
        {/await}

        <div class="ms-3 overflow-hidden">
            <p class="font-medium text-sm/[14px] md:text-sm whitespace-nowrap overflow-hidden animate-scroll-overflow-text">
                {titleLabel}
            </p>
            <p class="text-opacity-background-90 whitespace-nowrap overflow-hidden text-xs/[14px] pt-[4px] md:text-xs md:pt-0 animate-scroll-overflow-text">
                {mediumLabel}
            </p>
            <p class="text-xs/[14px] md:text-xs mt-[2px] text-opacity-background-90">
                {smallLabel}
            </p>
        </div>

        <div class="w-12 h-12 md:w-14 md:h-14 ps-2"></div>
    </div>

    <div class="absolute top-0 left-0 py-2 w-full">
        <div class="w-full grid grid-cols-[max-content_auto_max-content] music-item-play">
            <button class="w-12 h-12 md:w-14 md:h-14" onclick={addMusicAndPlay}>
                <div class="bg-black bg-opacity-40 grid box-border justify-items-center items-center rounded md:p-1">
                    <Icon type={IconType.Play}/>
                </div>
            </button>

            <div class="{folder ? 'cursor-pointer' : 'cursor-default'}" onclick={selectFolder}></div>

            <div class="w-12 h-12 md:w-14 md:h-14 ps-4">
                <button class="w-full h-full aspect-square" onclick={addMusic}>
                    <Icon type={IconType.QueuePlaylist} />
                </button>
            </div>
        </div>
    </div>
</div>

<style lang="scss">
  .music-item-play {
    opacity: 0;

    &:hover {
      animation-name: fadeIn;
      animation-duration: 0.5s;
      animation-fill-mode: forwards;
    }
  }
</style>