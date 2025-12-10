use image::{ImageBuffer, Rgba, RgbaImage};
use libblur::{AnisotropicRadius, BlurImageMut, ThreadingPolicy};
use rand::Rng;

const BACKGROUND_BLUR_RADIUS: u32 = 40;
pub fn generate_blurred_background(
    width: u32,
    height: u32,
    square_size: u32,
) -> Result<RgbaImage, libblur::BlurError> {
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

    blur_image(&mut img, BACKGROUND_BLUR_RADIUS)
}

fn blur_image(img: &mut RgbaImage, blur_radius: u32) -> Result<RgbaImage, libblur::BlurError> {
    let width = img.width();
    let height = img.height();
    let mut dst_img = BlurImageMut::borrow(
        img.as_mut(),
        width,
        height,
        libblur::FastBlurChannels::Channels4,
    );
    libblur::stack_blur(
        &mut dst_img,
        AnisotropicRadius::create(blur_radius, blur_radius),
        ThreadingPolicy::Adaptive,
    )?;

    // Optimize: Return the mutated image directly instead of cloning
    // The image is already modified in-place by stack_blur
    Ok(std::mem::take(img))
}
