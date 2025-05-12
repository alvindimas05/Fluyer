import { goto } from "$app/navigation";
import { isMacos } from "$lib/platform";
import { pageGotoRoute, pageGotoShow } from "$lib/stores/page";

const PageController = {
	goto: (route: string) => {
		if (isMacos()) {
			pageGotoRoute.set(route);
			pageGotoShow.set(false);
		} else {
			goto(route);
		}
	},
};

export default PageController;
