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

pub fn init() {
    #[cfg(target_os = "linux")]{
        // Set chromium as the webview backend
        std::env::set_var("WRY_WEBVIEW_BACKEND", "chromium");
        std::env::set_var("TAURI_WEBVIEW_BACKEND", "chromium");
        std::env::set_var("WEBVIEW_BACKEND", "chromium");

        // Enable hardware acceleration
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "0");
        std::env::set_var("WEBKIT_FORCE_HARDWARE_ACCELERATION", "1");

        // Chromium detection flags
        std::env::set_var("CHROME_DEVEL_SANDBOX", "/usr/lib/chromium-browser/chrome-sandbox");
        std::env::set_var("CHROME_WRAPPER", "/usr/bin/chromium-browser");
    }
}

// Re-export platform-specific constants and functions
