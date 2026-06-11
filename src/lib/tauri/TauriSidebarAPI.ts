import { listen } from '@tauri-apps/api/event';
import { TauriCommands } from '$lib/constants/TauriCommands';

export interface MouseLeavePayload {
	x: number;
	y: number;
}

const TauriSidebarAPI = {
	listenMouseLeave: (callback: (event: { payload: MouseLeavePayload }) => void) => {
		return listen<MouseLeavePayload>(TauriCommands.SIDEBAR_MOUSE_LEAVE, callback);
	}
};

export default TauriSidebarAPI;
