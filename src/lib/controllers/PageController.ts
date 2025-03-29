import { pageGotoRoute, pageGotoShow } from "$lib/stores/page";

const PageController = {
    goto: (route: string) => {
        pageGotoRoute.set(route);
        pageGotoShow.set(false);
    },
};

export default PageController;