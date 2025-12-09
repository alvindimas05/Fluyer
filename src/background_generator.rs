use image::{ImageBuffer, Rgba, RgbaImage};
use rand::Rng;

pub fn generate_blurred_background(
    monitor_width: u32,
    monitor_height: u32,
    scale_factor: f32,
    square_size: u32,
) -> RgbaImage {
    // Calculate downscaled dimensions
    let width = (monitor_width as f32 * scale_factor) as u32;
    let height = (monitor_height as f32 * scale_factor) as u32;

    // Create the image buffer
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    let mut rng = rand::rng();

    // Fill with random colored squares
    for y in (0..height).step_by(square_size as usize) {
        for x in (0..width).step_by(square_size as usize) {
            // Generate random color
            let r = rng.random::<u8>();
            let g = rng.random::<u8>();
            let b = rng.random::<u8>();
            let color = Rgba([r, g, b, 255]);

            // Fill the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    let px = x + dx;
                    let py = y + dy;
                    if px < width && py < height {
                        img.put_pixel(px, py, color);
                    }
                }
            }
        }
    }

    img
}