import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { TauriCommands } from '$lib/constants/TauriCommands';
import type { MusicPlayerSync } from '$lib/features/music/types';

export enum TauriMusicCommand {
	None = 'none',
	Pause = 'pause',
	Play = 'play',
	Next = 'next',
	Clear = 'clear',
	Repeat = 'repeat',
	RepeatOne = 'repeatOne',
	RepeatNone = 'repeatNone'
}

const TauriMusicAPI = {
	sendCommand: (command: TauriMusicCommand) => {
		return invoke(TauriCommands.MUSIC_CONTROLLER, { command });
	},
	setPosition: (position: number) => {
		return invoke(TauriCommands.MUSIC_POSITION_SET, {
			position: Math.trunc(position)
		});
	},
	requestSync: () => {
		return invoke(TauriCommands.MUSIC_PLAYER_REQUEST_SYNC);
	},
	setVolume: (volume: number) => {
		return invoke(TauriCommands.MUSIC_VOLUME_SET, { volume });
	},
	listenSync: (callback: (event: { payload: MusicPlayerSync }) => void) => {
		return listen<MusicPlayerSync>(TauriCommands.MUSIC_PLAYER_SYNC, callback);
	}
};

export default TauriMusicAPI;
