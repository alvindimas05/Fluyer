import Toastify from "toastify-js";

export enum ToastType {
	Info = "info",
	Error = "error",
}

const ToastController = {
	create: (message: string, type: ToastType) => {
		Toastify({
			text: message,
			duration: 3000,
			gravity: "top",
			position: "right",
			className: `toast toast-${type}`,
		}).showToast();
	},
	info: (message: string) => {
		ToastController.create(message, ToastType.Info);
	},
	error: (message: string) => {
		ToastController.create(message, ToastType.Error);
	},
};

export default ToastController;
