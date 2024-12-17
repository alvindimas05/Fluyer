use tauri_plugin_os::platform;

pub fn is_mobile() -> bool {
    matches!(platform(), "android" | "ios")
}
