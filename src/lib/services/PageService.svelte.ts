import { goto } from '$app/navigation';
import { page } from '$app/state';
import { PageRoutes } from '$lib/constants/PageRoutes';
import { isMobile } from '$lib/platform';
import { getCurrentWindow } from '@tauri-apps/api/window';

const PageService = {
	goTo: async (route: string) => {
		if (route === PageRoutes.PLAY && !isMobile()) {
			await getCurrentWindow().setFullscreen(true);
			return goto(route);
		}
		return goto(route);
	},
	back: async () => {
		if (page.url.pathname === PageRoutes.PLAY && !isMobile()) {
			await getCurrentWindow().setFullscreen(false);
		}
		history.back();
	}
};

export default PageService;
