use tauri_plugin_os::platform;

pub fn is_mobile() -> bool {
    matches!(platform(), "android" | "ios")
}

pub fn is_desktop() -> bool {
    !is_mobile()
}
