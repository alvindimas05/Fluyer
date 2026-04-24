use fast_image_resize::{images::Image, IntoImageView, Resizer};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};

pub fn compress_image(bytes: &[u8], size: u32) -> Result<Vec<u8>, String> {
    let rgba_image = image::load_from_memory(bytes)
        .map_err(|e| format!("Failed to decode image for resizing: {}", e))?
        .into_rgba8();
    let dynamic_image = DynamicImage::ImageRgba8(rgba_image);

    let mut dst_image = Image::new(
        size,
        size,
        dynamic_image
            .pixel_type()
            .unwrap_or(fast_image_resize::PixelType::U8x4),
    );
    let mut resizer = Resizer::new();
    resizer
        .resize(&dynamic_image, &mut dst_image, None)
        .map_err(|e| format!("Failed to resize image: {}", e))?;

    let raw_pixels = dst_image.into_vec();
    let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(size, size, raw_pixels)
        .ok_or_else(|| "Failed to create image buffer from raw pixels".to_string())?;

    let resized_dynamic = DynamicImage::ImageRgba8(img_buffer);

    let mut out_bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut out_bytes);
    resized_dynamic
        .write_to(&mut cursor, ImageFormat::Jpeg)
        .map_err(|e| format!("Failed to encode resized image: {}", e))?;

    Ok(out_bytes)
}
