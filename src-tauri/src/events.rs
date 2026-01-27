use tauri::{AppHandle, RunEvent, Window, WindowEvent};

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
            #[cfg(target_os = "macos")]
            {
                main_window()
                    .set_traffic_lights_inset(TRAFFIC_LIGHTS_INSET_X, TRAFFIC_LIGHTS_INSET_Y)
                    .unwrap();
            }
        }
        WindowEvent::Focused(focused) => {
            if *focused {
                crate::wgpu_renderer::trigger_redraw();
            }
        }
        WindowEvent::ThemeChanged(_) => {
            crate::wgpu_renderer::trigger_redraw();
        }
        _ => {}
    }
}

/// Handle application runtime events
pub fn handle_app_events(app_handle: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::Ready => {
            crate::state::initialize_on_ready(app_handle);
            crate::wgpu_renderer::start_render_loop(app_handle.clone());
        }
        RunEvent::WindowEvent {
            label: _,
            event: tauri::WindowEvent::Resized(size),
            ..
        } => {
            crate::wgpu_renderer::handle_wgpu_resize(app_handle, size.width, size.height);
        }
        _ => {}
    }
}
