// @ts-ignore
import { fluidScroll } from 'fluidscroll';
import musicStore from '$lib/stores/music.svelte';
import MusicPlayerService from '$lib/services/MusicPlayerService.svelte';
import ProgressService from '$lib/services/ProgressService.svelte';

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
			let elements = document.querySelectorAll('.animate-scroll-overflow-text');
			elements.forEach(el => {
				const htmlEl = el as HTMLElement;

				// Wrap contents in a span if not already wrapped
				if (!htmlEl.hasAttribute('data-scroll-wrapped')) {
					const span = document.createElement('span');
					span.style.display = 'inline-block';
					span.style.transition = `transform ${scrollDuration}ms linear`;
					// Move all child nodes (including Svelte's reactive text nodes) into the span
					while (htmlEl.firstChild) {
						span.appendChild(htmlEl.firstChild);
					}
					htmlEl.appendChild(span);
					htmlEl.setAttribute('data-scroll-wrapped', 'true');
				}

				const span = htmlEl.firstChild as HTMLElement;
				if (!span) return;

				// Calculate overflow
				const overflow = span.scrollWidth - htmlEl.clientWidth;

				if (overflow > 0) {
					if (scrollEnd) {
						span.style.transform = `translateX(-${overflow}px)`;
					} else {
						span.style.transform = `translateX(0px)`;
					}
				} else {
					// Reset if no longer overflowing
					span.style.transform = `translateX(0px)`;
				}
			});
			scrollEnd = !scrollEnd;
		}, scrollDuration + 2000);
	},
	blockDefaultKeyActions: () => {
		window.addEventListener('keydown', (e) => {
			if (['Tab', 'Escape', 'Space', 'Enter'].includes(e.key)) e.preventDefault();
		});
	},
	handleSpacePlaybackToggle: () => {
		document.addEventListener(
			'keydown',
			function (e) {
				if (e.code !== 'Space') return;

				const target = e.target as Element;

				if (target == document.body) e.preventDefault();

				if (target.matches('a, button, input, textarea, select')) {
					e.preventDefault();
					e.stopPropagation();
					// @ts-ignore
					target.blur();

					document.body.focus();
				}

				if (musicStore.isPlaying) {
					musicStore.isPlaying = false;
					MusicPlayerService.pause();
				} else {
					MusicPlayerService.play();
				}
			},
			true
		);
	}
};

export default UIInteractionService;
