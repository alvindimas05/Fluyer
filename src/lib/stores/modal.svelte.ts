import type { Modal } from '$lib/constants/Modal';

const modalStore = $state({
    show: false,
    data: null as any,
    type: null as Modal | null,
});

export default modalStore;
