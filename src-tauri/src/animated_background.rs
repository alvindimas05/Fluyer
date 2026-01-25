use image::{DynamicImage, Rgba, RgbaImage};
use rand::seq::IndexedRandom;
use std::cmp::max;
use tauri::async_runtime::block_on;

const SCALE: f32 = 0.05;
const CANVAS_BLOCK_SIZE: u32 = 200; // Avg of 100 and 150
const CANVAS_BLUR_RADIUS: u32 = 100;

pub fn generate_animated_background(
    img: &DynamicImage,
    target_width: u32,
    target_height: u32,
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    // 1. Extract Palette
    let img_buffer = img.to_rgba8();

    let palette = color_thief::get_palette(&img_buffer, color_thief::ColorFormat::Rgba, 10, 10)
        .expect("Failed to get palette");


    // 2. Generate Grid
    let scaled_width = (target_width as f32 * SCALE) as u32;
    let scaled_height = (target_height as f32 * SCALE) as u32;
    let block_size = (CANVAS_BLOCK_SIZE as f32 * SCALE) as u32;

    // Ensure minimum dimensions
    let scaled_width = max(1, scaled_width);
    let scaled_height = max(1, scaled_height);
    let block_size = max(1, block_size);

    let cols = (scaled_width as f32 / block_size as f32).ceil() as u32;
    let rows = (scaled_height as f32 / block_size as f32).ceil() as u32;

    let mut canvas = RgbaImage::new(scaled_width, scaled_height);
    let mut rng = rand::rng();

    for y in 0..rows {
        for x in 0..cols {
            let color = palette.choose(&mut rng).unwrap();

            let x_start = x * block_size;
            let y_start = y * block_size;

            // Fill block
            for by in 0..block_size {
                for bx in 0..block_size {
                    if x_start + bx < scaled_width && y_start + by < scaled_height {
                        canvas.put_pixel(
                            x_start + bx,
                            y_start + by,
                            Rgba([color.r, color.g, color.b, 255]),
                        );
                    }
                }
            }
        }
    }

    // 3. Blur
    let blur_radius = CANVAS_BLUR_RADIUS as f32 * SCALE;
    // Using image crate's gaussian blur for simplicity and quality
    let blurred = image::imageops::blur(&canvas, blur_radius);

    Ok(blurred)
}

#[tauri::command]
pub async fn update_animated_background(
    app_handle: tauri::AppHandle,
    audio_path: Option<String>,
) -> Result<(), String> {
    let app_handle_clone = app_handle.clone();
    let _ = tauri::async_runtime::spawn_blocking(move || {
        let bytes = if let Some(path) = audio_path {
            crate::debug!("Loading image from path: {}", path);
            let image_result = {
                #[cfg(not(target_os = "android"))]
                {
                    crate::music::metadata::MusicMetadata::get_image_from_path(path)
                }
                #[cfg(target_os = "android")]
                {
                    crate::music::metadata::MusicMetadata::get_image_from_path_android(path)
                }
            };
            block_on(image_result)
        } else {
            crate::debug!("Using default cover art");
            crate::music::metadata::MusicMetadata::get_default_cover_art()
                .map_err(|e| e.to_string())
        }?;

        let start = std::time::Instant::now();
        if let Ok(img) = image::load_from_memory(&bytes) {
            crate::debug!("Image loaded in {}ms", start.elapsed().as_millis());
            if let Ok(bg) = generate_animated_background(&img, 1920, 1080) {
                crate::wgpu_renderer::update_background(&app_handle_clone, bg);
            }
        } else {
            crate::debug!("Failed to load image from bytes");
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
