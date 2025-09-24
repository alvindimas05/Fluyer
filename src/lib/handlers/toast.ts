import { listen } from '@tauri-apps/api/event';
import ToastController, {ToastType} from "$lib/controllers/ToastController";

interface IToast {
    toastType: ToastType;
    message: string;
}

export default function toastHandler() {
    listen<IToast>('toast', (event) => {
        if(!event) return;
        console.log(event.payload);
        ToastController.create(event.payload.message, event.payload.toastType);
    });
}