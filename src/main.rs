// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
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

fn init_logging() {
    let mut config = ConfigBuilder::new();
    config.add_filter_ignore_str("symphonia_core");
    let config = config.build();

    let mut logs: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        simplelog::LevelFilter::Debug,
        config.clone(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )];

    // let log_file = File::create(log_path.clone());
    // if log_file.is_ok() {
    //     logs.push(WriteLogger::new(
    //         LevelFilter::Debug,
    //         config,
    //         log_file.unwrap(),
    //     ));
    // }

    CombinedLogger::init(logs).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    let mut backend = i_slint_backend_winit::Backend::new()?;
    backend.window_attributes_hook = Some(Box::new(|attributes| {
        attributes
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
    }));
    slint::platform::set_platform(Box::new(backend))?;

    let ui = AppWindow::new()?;

    ui.window().set_maximized(true);

    // Set traffic lights position after window is ready
    let ui_weak = ui.as_weak();
    ui.window().on_winit_window_event(move |_, event| {
        log::debug!("Received event in window event hook: {:?}", event);

        if event.eq(&WindowEvent::RedrawRequested) {
            if let Some(ui) = ui_weak.upgrade() {
                ui.window().with_winit_window(|window| {
                    set_traffic_lights_position(window, 12.0, 0.0);
                });
            }
        }

        EventResult::Propagate
    });

    let ui_weak_background = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        // Generate and set background image BEFORE showing/maximizing the window
        let background_img = generate_blurred_background(1920, 1080, 0.1, 20);

        let width = background_img.width();
        let height = background_img.height();
        let buffer = background_img.into_raw();

        let slint_image = slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(
            &buffer, width, height,
        ));

        if let Some(ui) = ui_weak_background.upgrade() {
            ui.set_background_image(slint_image);
        }
    })?;
    ui.run()?;
    Ok(())
}
