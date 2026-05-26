import { page } from '$app/state';
import { PageRoutes } from '$lib/constants/PageRoutes';
import { listen } from '@tauri-apps/api/event';

export enum ToastType {
	Info = 'info',
	Error = 'error'
}

export interface IToastAction {
	label: string;
	onClick: () => void;
}

export interface IToast {
	id: string;
	toastType: ToastType;
	message: string;
	duration?: number;
	action?: IToastAction | IToastAction[];
}

// Generate a random ID
function generateId() {
	return Math.random().toString(36).substring(2, 9);
}

class ToastServiceImpl {
	// Svelte 5 state for reactive list of toasts
	toasts = $state<IToast[]>([]);

	constructor() {}

	initialize() {
		return listen<IToast>('toast', (event) => {
			if (!event) return;
			// Tauri events typically just send type and message, so we map it here
			// Assuming the payload matches IToast structure roughly or we adapt
			this.create(event.payload.message, event.payload.toastType);
		});
	}

	create(
		message: string,
		type: ToastType,
		duration: number = 2000,
		action?: IToastAction | IToastAction[]
	) {
		const id = generateId();
		const toast: IToast = {
			id,
			message,
			toastType: type,
			duration,
			action
		};

		this.toasts.push(toast);

		// Limit to 3 toasts
		if (this.toasts.length > (page.url.pathname === PageRoutes.SETTINGS ? 1 : 3)) {
			this.toasts.shift();
		}

		if (duration > 0) {
			setTimeout(() => {
				this.remove(id);
			}, duration);
		}
	}

	remove(id: string) {
		const index = this.toasts.findIndex((t) => t.id === id);
		if (index !== -1) {
			this.toasts.splice(index, 1);
		}
	}

	clear() {
		this.toasts = [];
	}

	info(message: string, duration?: number, action?: IToastAction | IToastAction[]) {
		this.create(message, ToastType.Info, duration, action);
	}

	error(message: string, duration?: number, action?: IToastAction | IToastAction[]) {
		this.create(message, ToastType.Error, duration, action);
	}
}

const ToastService = new ToastServiceImpl();
export default ToastService;
