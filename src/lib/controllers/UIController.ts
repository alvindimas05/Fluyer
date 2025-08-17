// @ts-ignore
import { fluidScroll } from "fluidscroll";
import { equalizerShow } from "$lib/stores/equalizer";

const UIController = {
	initialize: async () => {
		UIController.listenAnimateScrollOverflowText();
		UIController.listenDisableKeyActions();
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
			if (['Tab', 'Escape', 'Space', 'Enter'].includes(e.key)) e.preventDefault();
		});
	},
	toggleEqualizer: (value: boolean) => {
		equalizerShow.set(value);
	},
};

export default UIController;
