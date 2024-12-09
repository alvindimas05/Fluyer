import { get } from "svelte/store";
import { loadingAll, loadingMusicList, loadingShow } from "$lib/stores/loading";

const LoadingController = {
	loadingMusicList: () => get(loadingMusicList),
	setLoadingMusicList: (value: boolean) => loadingMusicList.set(value),
	loadingAll: () => get(loadingAll),
	setLoadingAll: (value: boolean) => loadingAll.set(value),
	loadingShow: () => get(loadingShow),
	setLoadingShow: (value: boolean) => loadingShow.set(value),

	listen: () => {
		loadingMusicList.subscribe(LoadingController.onAllLoadingChange);
	},
	onAllLoadingChange: () => {
		if (LoadingController.loadingMusicList())
			LoadingController.setLoadingAll(true);
	},
};

export default LoadingController;
