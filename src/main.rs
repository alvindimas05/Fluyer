// Prevent console window in addition to Slint window in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod background_generator;
mod logging;
mod ui_setup;
mod window_utils;

use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    logging::init_logging();

    let backend = backend::configure_backend()?;
    slint::platform::set_platform(Box::new(backend))?;

    let ui = ui_setup::AppWindow::new()?;

    window_utils::setup_traffic_lights(&ui);
    ui_setup::setup_background(&ui)?;
    ui_setup::listen_resize(&ui);

    std::thread::spawn({
        let ui = ui.as_weak();
        move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            ui.upgrade_in_event_loop(move |ui| {
                ui.window().with_winit_window(|window| {
                    window.set_maximized(true);
                    ui.set_is_visible(true);
                });
            })
            .ok();
        }
    });

    // Run the application
    ui.run()?;
    Ok(())
}
