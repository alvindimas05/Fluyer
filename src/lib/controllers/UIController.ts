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

export function showThenFade(node: HTMLElement, options?: {
	initialVisibleDuration?: number,
	transitionInDuration?: number,
	transitionOutDuration?: number,
	transitionDelayDuration?: number,
}) {
	const {
		initialVisibleDuration = 2000,
		transitionInDuration = 500,
		transitionOutDuration = 1000,
		transitionDelayDuration = 2000,
	} = options || {};

	node.style.setProperty('--show-then-fade-transition-in-duration', `${transitionInDuration}ms`);
	node.style.setProperty('--show-then-fade-transition-delay-duration', `${transitionDelayDuration}ms`);
	node.style.setProperty('--show-then-fade-transition-out-duration', `${transitionOutDuration}ms`);
	node.classList.add('visible-initially');

	const timer = setTimeout(() => {
		node.classList.remove('visible-initially');
	}, initialVisibleDuration);

	return {
		destroy() {
			clearTimeout(timer);
			node.classList.remove('visible-initially');
		}
	};
}

export default UIController;
