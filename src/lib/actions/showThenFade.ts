export default function showThenFade(
	node: HTMLElement,
	options?: {
		initialVisibleDuration?: number;
		transitionInDuration?: number;
		transitionOutDuration?: number;
		transitionDelayDuration?: number;
	}
) {
	const {
		initialVisibleDuration = 2000,
		transitionInDuration = 500,
		transitionOutDuration = 1000,
		transitionDelayDuration = 2000
	} = options || {};

	node.style.setProperty('--show-then-fade-transition-in-duration', `${transitionInDuration}ms`);
	node.style.setProperty(
		'--show-then-fade-transition-delay-duration',
		`${transitionDelayDuration}ms`
	);
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
