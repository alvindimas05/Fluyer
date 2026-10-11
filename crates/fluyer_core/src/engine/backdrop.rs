//! Animated background input: cover palette and block artwork.

use super::FluyerEngine;
use crate::metadata::MusicMetadata;
use crate::services;
use std::sync::Arc;

impl FluyerEngine {
    /// Sharp block-colour square fed to the GPU backdrops instead of the real cover,
    /// so every palette colour gets equal share. Greys when nothing is playing.
    pub fn backdrop_block_artwork(&self) -> (Vec<u8>, u32, u32) {
        let colors = match self.player.get_current_track() {
            Some(ref t) => self.backdrop_palette(t),
            None => services::background::GREY_PALETTE.to_vec(),
        };
        let img = services::background::generate_block_artwork(
            &colors,
            services::background::BLOCK_ARTWORK_SIZE,
            services::background::BLOCK_ARTWORK_BLOCK,
        );
        let (w, h) = (img.width(), img.height());
        (img.into_raw(), w, h)
    }

    /// `backdrop_block_artwork` as JPEG for the Windows C ABI (byte buffers only).
    /// Blurred to mush downstream, so lossy encoding costs nothing visible.
    pub fn backdrop_block_artwork_jpeg(&self) -> Option<Vec<u8>> {
        let (rgba, w, h) = self.backdrop_block_artwork();
        rgba_to_jpeg(rgba, w, h)
    }

    // ponytail: dominant-color extraction decodes the full cover, so it is
    // memoized per track path. PlayView polls this on every state change.
    pub(super) fn backdrop_palette(&self, track: &MusicMetadata) -> Vec<[u8; 3]> {
        if let Some((cached_path, colors)) = self.palette_cache.read().unwrap().as_ref() {
            if cached_path == &track.path {
                return colors.as_ref().clone();
            }
        }

        let colors = match self.artwork_resolve_track(track, None) {
            Some(ref bytes) => services::background::extract_prominent_from_bytes(bytes, 10, false),
            None => services::background::GREY_PALETTE.to_vec(),
        };

        *self.palette_cache.write().unwrap() = Some((track.path.clone(), Arc::new(colors.clone())));
        colors
    }
}

fn rgba_to_jpeg(rgba: Vec<u8>, w: u32, h: u32) -> Option<Vec<u8>> {
    let rgb = image::DynamicImage::ImageRgba8(image::RgbaImage::from_raw(w, h, rgba)?).to_rgb8();
    let mut out = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 90)
        .encode_image(&rgb)
        .ok()?;
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grey_blocks_encode_as_jpeg_with_same_size() {
        let img = services::background::generate_block_artwork(
            &services::background::GREY_PALETTE,
            services::background::BLOCK_ARTWORK_SIZE,
            services::background::BLOCK_ARTWORK_BLOCK,
        );
        let (w, h) = (img.width(), img.height());
        let jpeg = rgba_to_jpeg(img.into_raw(), w, h).expect("encodes");
        assert_eq!(&jpeg[0..2], &[0xFF, 0xD8]);
        let back = image::load_from_memory(&jpeg).unwrap();
        assert_eq!((back.width(), back.height()), (w, h));
        assert!(rgba_to_jpeg(vec![0; 3], 1, 1).is_none(), "short buffer rejected");
    }
}
