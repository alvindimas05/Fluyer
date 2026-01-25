use image::ImageReader;
use image::{DynamicImage, Rgba, RgbaImage};
use rand::seq::IndexedRandom;
use std::cmp::max;
use std::io::Cursor;
use tauri::async_runtime::block_on;

const SCALE: f32 = 0.05;
const CANVAS_BLOCK_SIZE: u32 = 200; // Avg of 100 and 150
const CANVAS_BLUR_RADIUS: u32 = 125;

pub fn generate_animated_background(
    img: &DynamicImage,
    target_width: u32,
    target_height: u32,
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    // 1. Extract Palette
    let img_buffer = img.to_rgba8();

    let mut palette = color_thief::get_palette(&img_buffer, color_thief::ColorFormat::Rgba, 10, 10)
        .expect("Failed to get palette");

    for color in &mut palette {
        let (h, mut s, mut l) = rgb_to_hsl(color.r, color.g, color.b);

        if l > 0.5 {
            // Map lightness from [0.5, 1.0] to [0.25, 0.45] to keep it dark but preserve relative brightness
            l = 0.25 + (l - 0.5) * 0.4;

            // Boost saturation significantly for vibrance, but respect greyscale
            if s > 0.05 {
                s = (s.max(0.4) * 1.1).min(0.9);
            }

            let (r, g, b) = hsl_to_rgb(h, s, l);
            color.r = r;
            color.g = g;
            color.b = b;
        }
    }

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
        let start = std::time::Instant::now();
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
        crate::debug!("Cover art received in {}ms", start.elapsed().as_millis());

        let start = std::time::Instant::now();
        // Use format hint to speed up decoding - avoid auto-detection overhead
        let img_result = match image::guess_format(&bytes) {
            Ok(format) => {
                let mut reader = ImageReader::new(Cursor::new(&bytes));
                reader.set_format(format);
                reader.decode()
            }
            Err(_) => {
                // Fallback to auto-detection if format guess fails
                ImageReader::new(Cursor::new(&bytes))
                    .with_guessed_format()
                    .map_err(|e| e.to_string())?
                    .decode()
            }
        };

        if let Ok(img) = img_result {
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

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l > 0.5 {
        delta / (2.0 - max - min)
    } else {
        delta / (max + min)
    };

    let h = if max == r {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    } / 6.0;

    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}
