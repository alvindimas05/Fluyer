use tauri_plugin_os::platform;

/// macOS traffic light button inset from the left edge
pub const TRAFFIC_LIGHTS_INSET_X: f32 = 12.0;

/// macOS traffic light button inset from the top edge
pub const TRAFFIC_LIGHTS_INSET_Y: f32 = 20.0;

pub fn is_desktop() -> bool {
    platform() == "linux" || platform() == "macos" || platform() == "windows"
}

pub fn is_android() -> bool {
    platform() == "android"
}

pub fn is_ios() -> bool {
    platform() == "ios"
}

pub fn is_mobile() -> bool {
    is_android() || is_ios()
}
// Re-export platform-specific constants and functions
