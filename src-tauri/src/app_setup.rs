use crate::state::{initialize_store, GLOBAL_MAIN_WINDOW};
use tauri::{App, Manager};

#[cfg(target_os = "macos")]
use crate::platform::{TRAFFIC_LIGHTS_INSET_X, TRAFFIC_LIGHTS_INSET_Y};
#[cfg(target_os = "macos")]
use tauri_plugin_decorum::WebviewWindowExt;

/// Setup function called during app initialization
pub fn setup_application(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app
        .get_webview_window("main")
        .expect("Main window not found");

    configure_window(&main_window);

    GLOBAL_MAIN_WINDOW.set(main_window)
        .expect("Failed to set GLOBAL_MAIN_WINDOW");

    initialize_store(app);

    Ok(())
}

/// Configure window decorations and appearance based on platform
fn configure_window(window: &tauri::WebviewWindow) {
    #[cfg(any(windows, target_os = "linux"))]
    {
        window.set_decorations(false).unwrap();
        window.set_shadow(false).unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        window.make_transparent().unwrap();
        window
            .set_traffic_lights_inset(TRAFFIC_LIGHTS_INSET_X, TRAFFIC_LIGHTS_INSET_Y)
            .unwrap();
    }
}

/// Create the prevent default plugin based on build configuration
#[cfg(debug_assertions)]
pub fn prevent_default_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::debug())
        .build()
}

#[cfg(not(debug_assertions))]
pub fn prevent_default_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}