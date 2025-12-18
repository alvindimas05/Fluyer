// Prevent console window in addition to Slint window in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod background_generator;
mod logging;
mod music_scanner;
mod ui_setup;
mod window_utils;

use slint::ComponentHandle;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    logging::init_logging();

    let backend = backend::configure_backend()?;
    slint::platform::set_platform(Box::new(backend))?;

    let ui = ui_setup::AppWindow::new()?;

    #[cfg(target_os = "macos")]
    window_utils::setup_traffic_lights(&ui);
    ui_setup::setup_background(&ui)?;
    ui_setup::listen_resize(&ui);

    ui_setup::initialize(&ui);

    // Load music library
    ui_setup::load_music_library(&ui, "/Users/alvindimas05/Music/The Meaning of Life");

    // Run the application
    ui.run()?;
    Ok(())
}
