import {
    invoke,
    PermissionState,
    checkPermissions as checkPluginPermissions

} from '@tauri-apps/api/core'

export async function toast(value: string) {
    return await invoke<{ value?: string }>('plugin:fluyer|toast', {
        payload: {
            value,
        },
    });
}


export type PermissionStatus = {
    audio: PermissionState,
}

export type PermissionType = 'audio'

export async function checkPermissions(): Promise<PermissionStatus> {
    return await checkPluginPermissions('audio')
}

export async function requestPermissions(
    permissions: PermissionType[] | null
): Promise<PermissionStatus> {
    return await invoke('plugin:fluyer|request_permissions', {
        permissions
    })
}