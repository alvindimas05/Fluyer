// @ts-ignore
import {fluidScroll} from "fluidscroll";
import musicStore from "$lib/stores/music.svelte";
import MusicPlayerService from "$lib/services/MusicPlayerService.svelte";
import ProgressService from "$lib/services/ProgressService.svelte";

const UIInteractionService = {
    initialize: async () => {
        UIInteractionService.autoScrollOverflowText();
        UIInteractionService.blockDefaultKeyActions();
        UIInteractionService.handleSpacePlaybackToggle();
    },
    autoScrollOverflowText: () => {
        const scrollDuration = 3000;
        let scrollEnd = true;
        setInterval(() => {
            let elements = document.querySelectorAll(".animate-scroll-overflow-text");
            for (let i = elements.length; i > 0; --i) {
                fluidScroll({
                    scrollingElement: elements[i],
                    xPos: scrollEnd ? "end" : "start",
                    duration: scrollDuration,
                });
            }
            scrollEnd = !scrollEnd;
        }, scrollDuration + 2000);
    },
    blockDefaultKeyActions: () => {
        window.addEventListener("keydown", (e) => {
            if (["Tab", "Escape", "Space", "Enter"].includes(e.key))
                e.preventDefault();
        });
    },
    handleSpacePlaybackToggle: () => {
        document.addEventListener(
            "keydown",
            function (e) {
                if (e.code !== "Space") return;

                const target = e.target as Element;

                if (target == document.body) e.preventDefault();

                if (target.matches("a, button, input, textarea, select")) {
                    e.preventDefault();
                    e.stopPropagation();
                    // @ts-ignore
                    target.blur();

                    document.body.focus();
                }

                if (musicStore.isPlaying) {
                    musicStore.isPlaying = false;
                    MusicPlayerService.pause();
                    ProgressService.stop();
                } else {
                    MusicPlayerService.play();
                    ProgressService.start();
                }
            },
            true,
        );
    },
};

export default UIInteractionService;