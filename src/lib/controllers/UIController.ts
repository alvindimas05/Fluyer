// @ts-ignore
import { fluidScroll } from "fluidscroll";
import { equalizerShow } from "$lib/stores/equalizer";
import { get } from "svelte/store";
import { musicListType } from "$lib/stores/music";
import { MusicListType } from "$lib/home/music/types";
import FilterController from "$lib/controllers/FilterController";
import FolderController from "$lib/controllers/FolderController";
import MusicController from "./MusicController";

const UIController = {
	initialize: async () => {
		UIController.listenAnimateScrollOverflowText();
		UIController.listenDisableKeyActions();
		UIController.listenSpaceKeydown();
	},
	listenAnimateScrollOverflowText: () => {
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
	listenDisableKeyActions: () => {
		window.addEventListener("keydown", (e) => {
			if (["Tab", "Escape", "Space", "Enter"].includes(e.key))
				e.preventDefault();
		});
	},
	listenSpaceKeydown: () => {
		document.addEventListener(
			"keydown",
			function (e) {
				if (e.code !== "Space") return;

				const target = e.target;

				if (target == document.body) e.preventDefault();

				if (target.matches("a, button, input, textarea, select")) {
					e.preventDefault();
					e.stopPropagation();
					target.blur();

					document.body.focus();
				}

				if (MusicController.isPlaying) {
					MusicController.setIsPlaying(false);
					MusicController.pause();
				} else MusicController.play();
			},
			true,
		);
	},
	toggleEqualizer: (value: boolean) => {
		equalizerShow.set(value);
	},
	toggleMusicListType: async () => {
		const listType = get(musicListType);
		if (listType === MusicListType.All) {
			FilterController.setFilterAlbum(null);
			await FolderController.setMusicListToFolder();
		} else {
			FolderController.setFolder(null);
		}
		musicListType.set(
			listType === MusicListType.All ? MusicListType.Folder : MusicListType.All,
		);
	},
};

export function showThenFade(
	node: HTMLElement,
	options?: {
		initialVisibleDuration?: number;
		transitionInDuration?: number;
		transitionOutDuration?: number;
		transitionDelayDuration?: number;
	},
) {
	const {
		initialVisibleDuration = 2000,
		transitionInDuration = 500,
		transitionOutDuration = 1000,
		transitionDelayDuration = 2000,
	} = options || {};

	node.style.setProperty(
		"--show-then-fade-transition-in-duration",
		`${transitionInDuration}ms`,
	);
	node.style.setProperty(
		"--show-then-fade-transition-delay-duration",
		`${transitionDelayDuration}ms`,
	);
	node.style.setProperty(
		"--show-then-fade-transition-out-duration",
		`${transitionOutDuration}ms`,
	);
	node.classList.add("visible-initially");

	const timer = setTimeout(() => {
		node.classList.remove("visible-initially");
	}, initialVisibleDuration);

	return {
		destroy() {
			clearTimeout(timer);
			node.classList.remove("visible-initially");
		},
	};
}

export default UIController;
