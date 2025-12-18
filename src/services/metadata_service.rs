use crate::music_scanner::MusicMetadata;

/// MetadataService handles metadata formatting and transformation
pub struct MetadataService;

impl MetadataService {
    /// Create a new MetadataService instance
    pub fn new() -> Self {
        Self
    }

    /// Format metadata for display in music items
    /// Returns: (info string, metadata string)
    /// info: "Album - Artist" or just "Artist"
    /// metadata: "BitDepth/SampleRate Duration"
    pub fn format_for_display(metadata: &MusicMetadata) -> (String, String) {
        let info = if let Some(album) = &metadata.album {
            format!("{} - {}", album, metadata.artist)
        } else {
            metadata.artist.clone()
        };

        let metadata_str = format!(
            "{}/{} {}",
            metadata.bit_depth, metadata.sample_rate, metadata.duration
        );

        (info, metadata_str)
    }
}

impl Default for MetadataService {
    fn default() -> Self {
        Self::new()
    }
}
