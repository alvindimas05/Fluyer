use super::memory::bytes_into_c;
use crate::FluyerEngine;

/// Palette-block square (grey when nothing plays) as JPEG, the same format as the
/// cover thumbnails, so the Windows renderer decodes it like a cover. Random per
/// call: the UI calls it once per track. Free with `fluyer_bytes_free`.
#[no_mangle]
pub unsafe extern "C" fn fluyer_backdrop_get_block_artwork(
    engine: *mut FluyerEngine,
    out_len: *mut usize,
) -> *mut u8 {
    let bytes = engine.as_ref().and_then(|e| e.backdrop_block_artwork_jpeg());
    bytes_into_c(bytes, out_len)
}
