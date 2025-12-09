// Prevent console window in addition to Slint window in Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod background_generator;
mod window_utils;

use std::error::Error;

use background_generator::generate_blurred_background;
use i_slint_backend_winit::{EventResult, WinitWindowAccessor};
use simplelog::{CombinedLogger, ConfigBuilder, SharedLogger, TermLogger};
use window_utils::set_traffic_lights_position;
use winit::{event::WindowEvent, platform::macos::WindowAttributesExtMacOS};

slint::include_modules!();

/// Initialize logging configuration
fn init_logging() {
    let mut config = ConfigBuilder::new();
    config.add_filter_ignore_str("symphonia_core");
    let config = config.build();

    let logs: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        simplelog::LevelFilter::Debug,
        config,
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )];

    CombinedLogger::init(logs).expect("Failed to initialize logger");
}

/// Configure window backend with macOS-specific settings
fn configure_backend() -> Result<i_slint_backend_winit::Backend, Box<dyn Error>> {
    let mut backend = i_slint_backend_winit::Backend::new()?;
    backend.window_attributes_hook = Some(Box::new(|attributes| {
        attributes
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
            .with_transparent(true)
    }));
    Ok(backend)
}

/// Setup traffic light position adjustment on window redraw
fn setup_traffic_lights(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.window().on_winit_window_event(move |_, event| {
        if event.eq(&WindowEvent::RedrawRequested) {
            if let Some(ui) = ui_weak.upgrade() {
                ui.window().with_winit_window(|window| {
                    set_traffic_lights_position(window, 12.0, 0.0);
                });
            }
        }
        EventResult::Propagate
    });
}

/// Generate and set the background image
fn setup_background(ui: &AppWindow) -> Result<(), Box<dyn Error>> {
    let ui_weak = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        let background_img = generate_blurred_background(1920, 1080, 0.1, 20)
            .expect("Failed to generate background image");

        let width = background_img.width();
        let height = background_img.height();
        let buffer = background_img.into_raw();

        let slint_image = slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(
            &buffer, width, height,
        ));

        if let Some(ui) = ui_weak.upgrade() {
            ui.set_background_image(slint_image);
        }
    })?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    // Configure and set the backend
    let backend = configure_backend()?;
    slint::platform::set_platform(Box::new(backend))?;

    // Create and configure the UI
    let ui = AppWindow::new()?;
    ui.window().set_maximized(true);

    // Setup window customizations
    setup_traffic_lights(&ui);
    setup_background(&ui)?;

    // Run the application
    ui.run()?;
    Ok(())
}
