use crate::background_generator::generate_blurred_background;
use i_slint_backend_winit::WinitWindowAccessor;
use slint::{ComponentHandle, EventLoopError};

slint::include_modules!();
// Responsive rules: [min_width, column_count]
const RESPONSIVE_RULES: [[f64; 2]; 5] = [
    [1536.0, 7.0],
    [1280.0, 6.0],
    [1024.0, 5.0],
    [768.0, 4.0],
    [640.0, 3.0],
];

/// Generate and set the background image
pub fn setup_background(ui: &AppWindow) -> Result<(), EventLoopError> {
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
    })
}

pub fn listen_for_resize(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.window().on_winit_window_event(move |_window, event| {
        if let winit::event::WindowEvent::Resized(_) = event {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_sizing(&ui).expect("Failed to refresh sizing");
            }
        }
        i_slint_backend_winit::EventResult::Propagate
    })
}

fn refresh_sizing(ui: &AppWindow) -> Result<(), EventLoopError> {
    let ui_weak = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_weak.upgrade() {
            ui.window().with_winit_window(|window| {
                let scale_factor = window.scale_factor();
                let size = window.inner_size().to_logical(scale_factor);
                let width: f64 = size.width;

                let mut column_count = 2; // default fallback (< 640px)
                for rule in RESPONSIVE_RULES.iter() {
                    let min_width = rule[0];
                    let count = rule[1];

                    if width as f64 > min_width {
                        column_count = count as i32;
                        break;
                    }
                }

                let album_column_percentage = 1.0 / (column_count as f32);
                // Spacing 24px between columns. 0.5 is just extra unknown pixel (I had to adjust a bit to make it look right)
                let album_width = (width as f32 * album_column_percentage as f32).floor() - 24.5;
                ui.set_album_width(album_width);

                // Extra text and padding: 8 + 17 + 4 + 15 + 15 = 59px
                let album_height = album_width + 59.0;
                ui.set_album_height(album_height);

                let music_column_percentage = 1.0 / ((column_count / 2) as f32);
                // Spacing 24px between columns. 0.5 is just extra unknown pixel

                ui.set_music_list_column((column_count as f32 / 2.0).floor() as i32);
                ui.set_music_width((width as f32 * music_column_percentage).floor() - 24.5);
                ui.set_music_list_height(size.height as f32 - album_height);
            });
        }
    })
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
