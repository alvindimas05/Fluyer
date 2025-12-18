use crate::music_scanner::MusicScanner;
use image::GenericImageView;

/// Image processing constants
pub const THUMBNAIL_MAX_SIZE: u32 = 200;

/// ImageService handles all image-related operations
pub struct ImageService;

impl ImageService {
    /// Create a new ImageService instance
    pub fn new() -> Self {
        Self
    }

    /// Load image from bytes and convert to Slint Image
    pub fn load_from_bytes(&self, image_data: &[u8]) -> slint::Image {
        match image::load_from_memory(image_data) {
            Ok(img) => {
                let rgba_img = img.to_rgba8();
                let (width, height) = (rgba_img.width(), rgba_img.height());
                let pixel_buffer =
                    slint::SharedPixelBuffer::clone_from_slice(&rgba_img.into_raw(), width, height);
                slint::Image::from_rgba8(pixel_buffer)
            }
            Err(_) => slint::Image::default(),
        }
    }

    /// Resize image if it exceeds max size, maintaining aspect ratio
    pub fn resize(&self, img: image::DynamicImage) -> image::DynamicImage {
        let (width, height) = img.dimensions();

        if width > THUMBNAIL_MAX_SIZE || height > THUMBNAIL_MAX_SIZE {
            let scale = (THUMBNAIL_MAX_SIZE as f32 / width.max(height) as f32).min(1.0);
            let new_width = (width as f32 * scale) as u32;
            let new_height = (height as f32 * scale) as u32;

            img.resize(new_width, new_height, image::imageops::FilterType::Triangle)
        } else {
            img
        }
    }

    /// Load and resize image from bytes
    pub fn load_and_resize(&self, image_data: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
        match image::load_from_memory(image_data) {
            Ok(img) => {
                let resized_img = self.resize(img);
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

    /// Load cover image from file path, returning resized Slint Image
    pub fn load_cover_from_file(&self, file_path: &str) -> slint::Image {
        match self.extract_cover(file_path) {
            Ok(image_data) => self.load_from_bytes(&image_data),
            Err(_) => slint::Image::default(),
        }
    }

    /// Load cover image and resize it
    pub fn load_cover_resized(&self, file_path: &str) -> Option<(Vec<u8>, u32, u32)> {
        match self.extract_cover(file_path) {
            Ok(image_data) => self.load_and_resize(&image_data),
            Err(_) => None,
        }
    }

    /// Create Slint Image from raw RGBA data
    pub fn create_from_raw(&self, data: Vec<u8>, width: u32, height: u32) -> slint::Image {
        let pixel_buffer = slint::SharedPixelBuffer::clone_from_slice(&data, width, height);
        slint::Image::from_rgba8(pixel_buffer)
    }
}

impl Default for ImageService {
    fn default() -> Self {
        Self::new()
    }
}
