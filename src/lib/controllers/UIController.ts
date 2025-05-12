// @ts-ignore
import { fluidScroll } from "fluidscroll";

const UIController = {
	initialize: () => {
		UIController.listenAnimateScrollOverflowText();
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
};

export default UIController;
