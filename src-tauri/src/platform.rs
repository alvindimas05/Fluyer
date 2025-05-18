use tauri_plugin_os::platform;

pub fn is_android() -> bool {
    platform() == "android"
}

pub fn is_ios() -> bool {
    platform() == "ios"
}

pub fn is_mobile() -> bool {
    is_android() || is_ios()
}

pub fn is_desktop() -> bool {
    !is_mobile()
}