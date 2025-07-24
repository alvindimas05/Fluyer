import Toastify from "toastify-js";

const ToastController = {
	create: (message: string, type: string) => {
		Toastify({
			text: message,
			duration: 3000,
			gravity: "top",
			position: "right",
			className: `toast toast-${type}`,
		}).showToast();
	},
	info: (message: string) => {
		ToastController.create(message, "info");
	},
	error: (message: string) => {
		ToastController.create(message, "error");
	},
};

export default ToastController;
