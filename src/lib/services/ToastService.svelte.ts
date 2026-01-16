import Toastify from 'toastify-js';
import { listen } from '@tauri-apps/api/event';

export enum ToastType {
	Info = 'info',
	Error = 'error'
}

interface IToast {
	toastType: ToastType;
	message: string;
}

const ToastService = {
	initialize: () => {
		return listen<IToast>('toast', (event) => {
			if (!event) return;
			ToastService.create(event.payload.message, event.payload.toastType);
		});
	},
	create: (message: string, type: ToastType) => {
		Toastify({
			text: message,
			duration: 3000,
			gravity: 'top',
			position: 'right',
			className: `toast toast-${type}`
		}).showToast();
	},
	info: (message: string) => {
		ToastService.create(message, ToastType.Info);
	},
	error: (message: string) => {
		ToastService.create(message, ToastType.Error);
	}
};

export default ToastService;
