import { invoke } from '@tauri-apps/api/core'

export async function toast(value: string) {
    return await invoke<{ value?: string }>('plugin:fluyer|toast', {
        payload: {
            value,
        },
    });
}
