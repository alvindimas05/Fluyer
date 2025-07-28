// @ts-ignore
import { fluidScroll } from "fluidscroll";
import {equalizerShow} from "$lib/stores/equalizer";

const UIController = {
	initialize: async () => {
		UIController.listenAnimateScrollOverflowText();
		UIController.listenDisableTabSelection();
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
	listenDisableTabSelection: () => {
		window.addEventListener("keydown", (e) => {
			if (e.key !== "Tab") return;
			e.preventDefault();
		});
	},
	toggleEqualizer: (value: boolean) => {
		equalizerShow.set(value);
	}
};

export default UIController;
