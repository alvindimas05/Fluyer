use crate::music_scanner::MusicScanner;
use image::GenericImageView;

/// ImageService handles all image-related operations
pub struct ImageService;

impl ImageService {
    /// Create a new ImageService instance
    pub fn new() -> Self {
        Self
    }

    /// Resize image if it exceeds max size, maintaining aspect ratio
    pub fn resize(&self, img: image::DynamicImage, max_size: u32) -> image::DynamicImage {
        let (width, height) = img.dimensions();

        if width > max_size || height > max_size {
            let scale = (max_size as f32 / width.max(height) as f32).min(1.0);
            let new_width = (width as f32 * scale) as u32;
            let new_height = (height as f32 * scale) as u32;

            img.resize(new_width, new_height, image::imageops::FilterType::Triangle)
        } else {
            img
        }
    }

    /// Load and resize image from bytes
    pub fn load_and_resize(&self, image_data: &[u8], max_size: u32) -> Option<(Vec<u8>, u32, u32)> {
        match image::load_from_memory(image_data) {
            Ok(img) => {
                let resized_img = self.resize(img, max_size);
                let rgba_img = resized_img.to_rgba8();
                let width = rgba_img.width();
                let height = rgba_img.height();
                Some((rgba_img.into_raw(), width, height))
            }
            Err(_) => None,
        }
    }

    /// Extract cover art from a music file
    pub fn extract_cover(&self, file_path: &str) -> Result<Vec<u8>, String> {
        let scanner = MusicScanner::new();
        scanner.extract_cover_to_memory(file_path)
    }

    /// Load cover image and resize it
    pub fn load_cover_resized(
        &self,
        file_path: &str,
        max_size: u32,
    ) -> Option<(Vec<u8>, u32, u32)> {
        match self.extract_cover(file_path) {
            Ok(image_data) => self.load_and_resize(&image_data, max_size),
            Err(_) => None,
        }
    }

    pub fn load_cover_as_slint_image(&self, file_path: &str, max_size: u32) -> slint::Image {
        match self.load_cover_resized(file_path, max_size) {
            Some((image_data, width, height)) => {
                let pixel_buffer =
                    slint::SharedPixelBuffer::clone_from_slice(&image_data, width, height);
                slint::Image::from_rgba8(pixel_buffer)
            }
            None => slint::Image::default(),
        }
    }

    // Create Slint Image from raw RGBA data
    // pub fn create_from_raw(&self, data: Vec<u8>, width: u32, height: u32) -> slint::Image {
    //     let pixel_buffer = slint::SharedPixelBuffer::clone_from_slice(&data, width, height);
    //     slint::Image::from_rgba8(pixel_buffer)
    // }
}

impl Default for ImageService {
    fn default() -> Self {
        Self::new()
    }
}
