import { platform } from "@tauri-apps/plugin-os";

export function isMobile(): boolean {
    return ['android', 'ios'].includes(platform());
}