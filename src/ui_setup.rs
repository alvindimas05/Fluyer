use crate::background_generator::generate_blurred_background;
use i_slint_backend_winit::WinitWindowAccessor;
use slint::{ComponentHandle, EventLoopError};
use std::error::Error;

slint::include_modules!();

/// Generate and set the background image
pub fn setup_background(ui: &AppWindow) -> Result<(), Box<dyn Error>> {
    let ui_weak = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        let background_img =
            generate_blurred_background((1920.0 * 0.1) as u32, (1080.0 * 0.1) as u32, 20)
                .expect("Failed to generate background image");

        let width = background_img.width();
        let height = background_img.height();

        // Optimize: Move buffer directly into SharedPixelBuffer without intermediate clone
        let buffer = background_img.into_raw();
        let pixel_buffer = slint::SharedPixelBuffer::clone_from_slice(&buffer, width, height);

        // Create image from the shared buffer
        let slint_image = slint::Image::from_rgba8(pixel_buffer);

        if let Some(ui) = ui_weak.upgrade() {
            ui.set_background_image(slint_image);
        }
    })?;
    Ok(())
}

/// Maximize window after event loop starts
pub fn setup_maximize(ui: &AppWindow) -> Result<(), EventLoopError> {
    let ui_weak = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_weak.upgrade() {
            ui.window().with_winit_window(|window| {
                window.set_maximized(true);
            });
        }
    })
}
