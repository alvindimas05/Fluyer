// use fast_image_resize::{IntoImageView, ResizeAlg, ResizeOptions, Resizer};
// use fast_image_resize::images::Image;
// use image::{ImageEncoder, ImageFormat, ExtendedColorType};

pub struct ImageHandler {}

impl ImageHandler {
    // pub fn resize_image_to_base64(
    //     image_data: &[u8],
    //     size: Option<String>,
    // ) -> Option<String> {
    //     if size.is_none() {
    //         return Self::encode_standard_to_base64(image_data);
    //     }

    //     // Detect original format
    //     let format = image::guess_format(image_data).ok()?;

    //     let image_size = size.unwrap().parse::<u32>().unwrap();

    //     let src_image = image::load_from_memory(&image_data).ok()?;

    //     // Create container for data of destination image
    //     let mut dst_image = Image::new(
    //         image_size,
    //         image_size,
    //         src_image.pixel_type().unwrap(),
    //     );

    //     let mut resizer = Resizer::new();
    //     resizer.resize(&src_image, &mut dst_image, &ResizeOptions::default()
    //         .resize_alg(ResizeAlg::Nearest)).ok()?;

    //     // Encode dynamically
    //     let mut result_buf = Vec::new();

    //     let color_type: ExtendedColorType = src_image.color().into(); // important!

    //     match format {
    //         ImageFormat::Png => {
    //             image::codecs::png::PngEncoder::new(&mut result_buf)
    //                 .write_image(
    //                     dst_image.buffer(),
    //                     image_size,
    //                     image_size,
    //                     color_type,
    //                 )
    //                 .unwrap();
    //         }
    //         _ => {
    //             // Default JPEG
    //             image::codecs::jpeg::JpegEncoder::new(&mut result_buf)
    //                 .write_image(
    //                     dst_image.buffer(),
    //                     image_size,
    //                     image_size,
    //                     color_type,
    //                 )
    //                 .unwrap();
    //         }
    //     }

    //     Self::encode_standard_to_base64(&result_buf)
    // }

    pub fn encode_to_base64(data: &[u8]) -> Option<String> {
        Some(base64_simd::STANDARD.encode_to_string(data))
    }
}
