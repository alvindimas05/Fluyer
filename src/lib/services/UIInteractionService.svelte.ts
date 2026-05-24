// @ts-ignore
import { fluidScroll } from 'fluidscroll';
import musicStore from '$lib/stores/music.svelte';
import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
import ProgressService from '$lib/services/ProgressService.svelte';
import modalStore from '$lib/stores/modal.svelte';

const UIInteractionService = {
	initialize: async () => {
		UIInteractionService.autoScrollOverflowText();
		UIInteractionService.handleKeyControls();
	},
	// FIXME: Auto scroll overflow eating CPU
	autoScrollOverflowText: () => {
		// const scrollDuration = 3000;
		// let scrollEnd = true;
		// setInterval(() => {
		// 	let elements = document.querySelectorAll('.animate-scroll-overflow-text');
		// 	elements.forEach(el => {
		// 		const htmlEl = el as HTMLElement;
		// 		// Wrap contents in a span if not already wrapped
		// 		if (!htmlEl.hasAttribute('data-scroll-wrapped')) {
		// 			const span = document.createElement('span');
		// 			span.style.display = 'inline-block';
		// 			span.style.transition = `transform ${scrollDuration}ms linear`;
		// 			// Move all child nodes (including Svelte's reactive text nodes) into the span
		// 			while (htmlEl.firstChild) {
		// 				span.appendChild(htmlEl.firstChild);
		// 			}
		// 			htmlEl.appendChild(span);
		// 			htmlEl.setAttribute('data-scroll-wrapped', 'true');
		// 		}
		// 		const span = htmlEl.firstChild as HTMLElement;
		// 		if (!span) return;
		// 		// Calculate overflow
		// 		const overflow = span.scrollWidth - htmlEl.clientWidth;
		// 		if (overflow > 0) {
		// 			if (scrollEnd) {
		// 				span.style.transform = `translateX(-${overflow}px)`;
		// 			} else {
		// 				span.style.transform = `translateX(0px)`;
		// 			}
		// 		} else {
		// 			// Reset if no longer overflowing
		// 			span.style.transform = `translateX(0px)`;
		// 		}
		// 	});
		// 	scrollEnd = !scrollEnd;
		// }, scrollDuration + 2000);
	},
	handleKeyControls: () => {
		document.addEventListener(
			'keydown',
			function (e) {
				const target = e.target as Element;
				const handledKeys = ['Space', 'Tab', 'Escape'];

				if (target == document.body && handledKeys.includes(e.code || e.key)) e.preventDefault();

				if (e.code === 'Space') {
					if (target.matches('a, button, select')) {
						e.preventDefault();
						e.stopPropagation();
						// @ts-ignore
						target.blur();

						document.body.focus();
					}
					if (target.matches('input, textarea')) return;

					if (musicStore.isPlaying) {
						musicStore.isPlaying = false;
						MusicPlayerService.pause();
					} else {
						MusicPlayerService.play();
					}
				} else if (['Tab', 'Escape'].includes(e.key)) {
					if (target.matches('a, button, select')) {
						e.preventDefault();
						e.stopPropagation();
						// @ts-ignore
						target.blur();

						document.body.focus();
					}
					if (target.matches('input, textarea')) return;
				}
			},
			true
		);
	}
};

export default UIInteractionService;
