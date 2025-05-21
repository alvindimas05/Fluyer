import { goto } from "$app/navigation";

const PageController = {
	goto: (route: string) => {
		goto(route);
	},
	back: () => {
		history.back();
	},
};

export default PageController;
