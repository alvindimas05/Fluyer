use crate::background_generator::generate_blurred_background;
use crate::music_scanner::MusicScanner;
use i_slint_backend_winit::WinitWindowAccessor;
use log::{error, info};
use slint::{ComponentHandle, EventLoopError, Model, ModelRc, VecModel};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

pub fn listen_resize(ui: &AppWindow) {
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

                // Extra text and padding: 8 + 17 + 4 + 15 = 44px
                // 15px is for extra height to prevent vertical scrollbar
                let album_height = album_width + 44.0 + 15.0;
                ui.set_album_height(album_height);

                let music_column_percentage = 1.0 / ((column_count / 2) as f32);
                ui.set_music_list_column((column_count as f32 / 2.0).floor() as i32);
                ui.set_music_width(music_column_percentage * 100.0);
                ui.set_music_list_height(size.height as f32 - album_height);
            });
        }
    })
}

/// Load music library from a specific directory
pub fn load_music_library(ui: &AppWindow, music_dir: &str) {
    info!("Loading music library from: {}", music_dir);

    let music_dir = music_dir.to_string();
    let ui_weak = ui.as_weak();

    // Scan in background thread to avoid blocking UI
    std::thread::spawn(move || {
        let scanner = MusicScanner::new();
        let music_list = scanner.scan_directory(&music_dir);

        info!("Loaded {} songs from library", music_list.len());

        // Store music metadata for progressive loading
        let music_list = Arc::new(music_list);
        let music_list_clone = Arc::clone(&music_list);

        // Update UI in event loop
        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                // Initially create items without images (for performance)
                let items: Vec<MusicItemData> = music_list
                    .iter()
                    .map(|metadata| {
                        let info = format!("{} - {}", metadata.album, metadata.artist);
                        let metadata_str = format!(
                            "{}/{} {}",
                            metadata.bit_depth, metadata.sample_rate, metadata.duration
                        );

                        MusicItemData {
                            cover_image: slint::Image::default(), // Will be loaded progressively
                            title: metadata.title.clone().into(),
                            info: info.into(),
                            metadata: metadata_str.into(),
                        }
                    })
                    .collect();

                let model = Rc::new(VecModel::from(items));
                ui.set_music_items(ModelRc::from(model));

                // Setup progressive image loading
                setup_progressive_loading(&ui, music_list_clone);
            }
        })
        .ok();
    });
}

/// Setup progressive loading with debouncing
fn setup_progressive_loading(
    ui: &AppWindow,
    music_list: Arc<Vec<crate::music_scanner::MusicMetadata>>,
) {
    let last_request_time = Arc::new(Mutex::new(Instant::now()));
    let pending_range = Arc::new(Mutex::new(Option::<(i32, i32)>::None));
    let ui_weak = ui.as_weak();

    // Handle image requests with debouncing
    ui.on_request_images(move |start_idx, end_idx| {
        // Update pending range
        *pending_range.lock().unwrap() = Some((start_idx, end_idx));
        *last_request_time.lock().unwrap() = Instant::now();

        // Spawn debounce thread
        let pending_range = Arc::clone(&pending_range);
        let last_request_time = Arc::clone(&last_request_time);
        let ui_weak = ui_weak.clone();
        let music_list = Arc::clone(&music_list);

        std::thread::spawn(move || {
            // Wait for 200ms debounce
            std::thread::sleep(Duration::from_millis(200));

            // Check if this is still the latest request
            let elapsed = last_request_time.lock().unwrap().elapsed();
            if elapsed < Duration::from_millis(200) {
                return; // A newer request came in, abort this one
            }

            // Get the pending range
            let range = pending_range.lock().unwrap().take();
            if let Some((start, end)) = range {
                load_images_for_range(ui_weak.clone(), Arc::clone(&music_list), start, end);
            }
        });
    });
}

/// Load images for a specific range of items
fn load_images_for_range(
    ui_weak: slint::Weak<AppWindow>,
    music_list: Arc<Vec<crate::music_scanner::MusicMetadata>>,
    start_idx: i32,
    end_idx: i32,
) {
    info!("Loading images for range {} to {}", start_idx, end_idx);

    // First, check which images need to be loaded (must be done in event loop)
    let ui_weak_clone = ui_weak.clone();
    let music_list_clone = Arc::clone(&music_list);

    slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_weak_clone.upgrade() {
            let model = ui.get_music_items();
            let start = start_idx.max(0) as usize;
            let end = (end_idx as usize).min(music_list_clone.len().saturating_sub(1));

            let mut indices_to_load = Vec::new();

            for idx in start..=end {
                if idx >= music_list_clone.len() {
                    break;
                }

                // Check if the image is already loaded
                if idx < model.row_count() {
                    if let Some(item) = model.row_data(idx) {
                        // Check if cover_image is empty/default
                        if item.cover_image.size().width == 0 {
                            indices_to_load.push(idx);
                        }
                    }
                }
            }

            info!(
                "Need to load {} images out of {} in range",
                indices_to_load.len(),
                end - start + 1
            );

            // Now extract cover images in background thread
            if !indices_to_load.is_empty() {
                let ui_weak_for_thread = ui_weak.clone();
                let music_list_for_thread = Arc::clone(&music_list);

                std::thread::spawn(move || {
                    let scanner = crate::music_scanner::MusicScanner::new();
                    let mut loaded_images = Vec::new();

                    // Only extract cover art for items that need it
                    for idx in indices_to_load {
                        let metadata = &music_list_for_thread[idx];

                        // Try to extract cover art
                        match scanner.extract_cover_to_memory(&metadata.file_path) {
                            Ok(image_data) => {
                                // Load image data into image
                                match image::load_from_memory(&image_data) {
                                    Ok(img) => {
                                        // Convert to RGBA8
                                        let rgba_img = img.to_rgba8();
                                        let width = rgba_img.width();
                                        let height = rgba_img.height();

                                        loaded_images.push((
                                            idx,
                                            rgba_img.into_raw(),
                                            width,
                                            height,
                                        ));
                                    }
                                    Err(e) => {
                                        error!(
                                            "Failed to load cover image for {}: {}",
                                            metadata.title, e
                                        );
                                    }
                                }
                            }
                            Err(_) => {
                                // No cover art available or extraction failed - skip silently
                            }
                        }
                    }

                    info!(
                        "Loaded {} cover images for range {} to {}",
                        loaded_images.len(),
                        start_idx,
                        end_idx
                    );

                    // Update UI with loaded images
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak_for_thread.upgrade() {
                            let model = ui.get_music_items();

                            for (idx, image_data, width, height) in loaded_images {
                                if idx < model.row_count() {
                                    let mut item = model.row_data(idx).unwrap();

                                    // Create Slint image from RGBA data
                                    let pixel_buffer = slint::SharedPixelBuffer::clone_from_slice(
                                        &image_data,
                                        width,
                                        height,
                                    );
                                    item.cover_image = slint::Image::from_rgba8(pixel_buffer);

                                    // Update the model
                                    model.set_row_data(idx, item);
                                }
                            }
                        }
                    })
                    .ok();
                });
            }
        }
    })
    .ok();
}

/// Maximize window after event loop starts
pub fn initialize(ui: &AppWindow) {
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
}
