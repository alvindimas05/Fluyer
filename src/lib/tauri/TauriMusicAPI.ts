import { invoke } from '@tauri-apps/api/core';
import { CommandRoutes } from '$lib/constants/CommandRoutes';

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
		return invoke(CommandRoutes.MUSIC_CONTROLLER, { command });
	},
	setPosition: (position: number) => {
		return invoke(CommandRoutes.MUSIC_POSITION_SET, {
			position: Math.trunc(position)
		});
	},
	requestSync: () => {
		return invoke(CommandRoutes.MUSIC_PLAYER_REQUEST_SYNC);
	},
	setVolume: (volume: number) => {
		return invoke(CommandRoutes.MUSIC_SET_VOLUME, { volume });
	}
};

export default TauriMusicAPI;
