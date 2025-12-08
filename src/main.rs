// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window_utils;

use std::error::Error;

use i_slint_backend_winit::WinitWindowAccessor;
use window_utils::set_traffic_lights_position;
use winit::platform::macos::WindowAttributesExtMacOS;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let mut backend = i_slint_backend_winit::Backend::new()?;
    backend.window_attributes_hook = Some(Box::new(|attributes| {
        attributes
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
    }));
    slint::platform::set_platform(Box::new(backend))?;

    let ui = AppWindow::new()?;

    slint::invoke_from_event_loop({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.window().with_winit_window(|window| {
                set_traffic_lights_position(window, 12.0, 0.0);
            });
        }
    })?;

    ui.window().set_maximized(true);

    ui.run()?;
    Ok(())
}
