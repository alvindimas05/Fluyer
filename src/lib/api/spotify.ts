import type { MusicData } from "$lib/home/music/types";
import { spotifyAccessToken } from "$lib/stores";
import { invoke } from "@tauri-apps/api/core";
import axios, { AxiosHeaders } from "axios";
import { get } from "svelte/store";

export default class SpotifyApi {
	apiEndpoint = import.meta.env.VITE_SPOTIFY_API_ENDPOINT;
	authEndpoint = import.meta.env.VITE_SPOTIFY_AUTH_ENDPOINT;
	clientId = import.meta.env.VITE_SPOTIFY_CLIENT_ID;
	clientSecret = import.meta.env.VITE_SPOTIFY_CLIENT_SECRET;

	get headers() {
		return {
			Authorization: `Bearer ${get(spotifyAccessToken)}`,
			"Content-Type": "application/x-www-form-urlencoded",
		};
	}

	public async auth() {
		if (get(spotifyAccessToken) !== null) return;

		const authorization = btoa(`${this.clientId}:${this.clientSecret}`);
		try {
			const res = await axios.post(
				this.authEndpoint,
				{ grant_type: "client_credentials" },
				{
					headers: {
						Authorization: `Basic ${authorization}`,
						"Content-Type": "application/x-www-form-urlencoded",
					},
				},
			);
			spotifyAccessToken.set(res.data.access_token);
		} catch (err) {
			console.error(err);
		}
	}

	public async searchMusic(music: MusicData): Promise<SpotifyMusic | null> {
		await this.auth();

		const url = new URL(`${this.apiEndpoint}/search`);
		url.searchParams.set("type", "album");
		url.searchParams.set("limit", "1");
		url.searchParams.set("q", `${music.album || music.title} ${music.artist}`);

		try {
			const res = await axios.get(url.toString(), { headers: this.headers });
			if (res.data.albums.items.length < 1) return null;

			return new SpotifyMusic(res.data.albums.items[0]);
		} catch (err) {
			console.error(err);
		}
		return null;
	}
}

class SpotifyMusic {
	data: any;
	constructor(data: any) {
		this.data = data;
	}

	get imageUrl() {
		return this.data.images[0].url;
	}
}
