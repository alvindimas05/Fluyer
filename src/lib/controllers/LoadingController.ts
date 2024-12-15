import { get } from "svelte/store";
import { loadingBackground, loadingMusicList, loadingShow } from "$lib/stores/loading";

const LoadingController = {
	loadingMusicList: () => get(loadingMusicList),
	setLoadingMusicList: (value: boolean) => loadingMusicList.set(value),
	loadingBackground: () => get(loadingBackground),
	setLoadingBackground: (value: boolean) => loadingBackground.set(value),

	loadingShow: () => get(loadingShow),
	setLoadingShow: (value: boolean) => loadingShow.set(value),

	listen: () => {
		loadingMusicList.subscribe(LoadingController.onAllLoadingChange);
	},
	onAllLoadingChange: () => {
		if (LoadingController.loadingMusicList() && LoadingController.loadingBackground())
			LoadingController.setLoadingShow(true);
	},
};

export default LoadingController;
