import { listen } from '@tauri-apps/api/event';

export enum ToastType {
	Info = 'info',
	Error = 'error'
}

export interface IToast {
	id: string;
	toastType: ToastType;
	message: string;
	duration?: number;
}

// Generate a random ID
function generateId() {
	return Math.random().toString(36).substring(2, 9);
}

class ToastServiceImpl {
	// Svelte 5 state for reactive list of toasts
	toasts = $state<IToast[]>([]);

	constructor() { }

	initialize() {
		return listen<IToast>('toast', (event) => {
			if (!event) return;
			// Tauri events typically just send type and message, so we map it here
			// Assuming the payload matches IToast structure roughly or we adapt
			this.create(event.payload.message, event.payload.toastType);
		});
	}

	create(message: string, type: ToastType, duration: number = 2000) {
		const id = generateId();
		const toast: IToast = {
			id,
			message,
			toastType: type,
			duration
		};

		this.toasts.push(toast);

		// Limit to 3 toasts
		if (this.toasts.length > 3) {
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

	info(message: string, duration?: number) {
		this.create(message, ToastType.Info, duration);
	}

	error(message: string, duration?: number) {
		this.create(message, ToastType.Error, duration);
	}
}

const ToastService = new ToastServiceImpl();
export default ToastService;
