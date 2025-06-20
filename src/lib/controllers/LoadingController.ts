import { get } from "svelte/store";
import {
	loadingBackground,
	loadingMusicList,
	loadingShow,
} from "$lib/stores/loading";
import MusicController from "./MusicController";

const LoadingController = {
	loadingMusicList: () => get(loadingMusicList),
	setLoadingMusicList: (value: boolean) => loadingMusicList.set(value),
	loadingBackground: () => get(loadingBackground),
	setLoadingBackground: (value: boolean) => loadingBackground.set(value),

	loadingShow: () => get(loadingShow),
	setLoadingShow: (value: boolean) => loadingShow.set(value),

	listen: () => {
		const unlistenLoadingBackground = loadingBackground.subscribe((value) => {
			if (!value) return;
			MusicController.getMusics();
			unlistenLoadingBackground();
		});
		const unlistenLoadingMusicList = loadingMusicList.subscribe(() => {
			if (
				LoadingController.loadingMusicList() &&
				LoadingController.loadingBackground()
			) {
				LoadingController.setLoadingShow(true);
				unlistenLoadingMusicList();
			}
		});
	},
};

export default LoadingController;
