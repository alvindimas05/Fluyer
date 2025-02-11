import { backgroundIsLight } from "$lib/stores/background";

const BackgroundController = {
	setIsLight: (value: boolean) => {
		backgroundIsLight.set(value);
	},
};

export default BackgroundController;
