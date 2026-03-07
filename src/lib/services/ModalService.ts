import modalStore from '$lib/stores/modal.svelte';
import { Modal } from '$lib/constants/Modal';

const ModalService = {
    open(id: Modal, data?: any) {
        modalStore.show = true;
        modalStore.type = id;
        modalStore.data = data;
    },

    close() {
        modalStore.show = false;
        modalStore.type = null;
        modalStore.data = null;
    },
    show() {
        return modalStore.show;
    },
}

export default ModalService;
