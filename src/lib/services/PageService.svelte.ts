import { goto } from '$app/navigation';

const PageService = {
	goTo: (route: string) => {
		return goto(route);
	},
	back: () => {
		history.back();
	}
};

export default PageService;
