use tauri::{Window, WindowEvent};

#[cfg(target_os = "macos")]
use crate::platform::{TRAFFIC_LIGHTS_INSET_X, TRAFFIC_LIGHTS_INSET_Y};
#[cfg(target_os = "macos")]
use crate::state::main_window;
#[cfg(target_os = "macos")]
use tauri_plugin_decorum::WebviewWindowExt;

/// Handle window events
pub fn handle_window_events(_: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::Resized(_) => {
            handle_window_resize();
        }
        _ => {}
    }
}

/// Handle window resize events
fn handle_window_resize() {
    #[cfg(target_os = "macos")]
    {
        main_window()
            .set_traffic_lights_inset(TRAFFIC_LIGHTS_INSET_X, TRAFFIC_LIGHTS_INSET_Y)
            .unwrap();
    }
}
