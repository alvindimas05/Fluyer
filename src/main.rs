// Prevent console window in addition to Slint window in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod background_generator;
mod logging;
mod ui_setup;
mod window_utils;

use slint::ComponentHandle;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    logging::init_logging();

    // Configure and set the backend
    let backend = backend::configure_backend()?;
    slint::platform::set_platform(Box::new(backend))?;

    // Create the UI
    let ui = ui_setup::AppWindow::new()?;

    // Setup window customizations
    window_utils::setup_traffic_lights(&ui);
    ui_setup::setup_background(&ui)?;
    ui_setup::setup_maximize(&ui)?;
    ui_setup::listen_for_resize(&ui);

    // Run the application
    ui.run()?;
    Ok(())
}
