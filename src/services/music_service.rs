use crate::music_scanner::{GroupedMusic, MusicMetadata, MusicScanner};

/// MusicService handles all music-related operations
pub struct MusicService;

impl MusicService {
    /// Create a new MusicService instance
    pub fn new() -> Self {
        Self
    }

    /// Scan a directory for music files
    pub fn scan_directory(&self, path: &str) -> Vec<MusicMetadata> {
        let scanner = MusicScanner::new();
        scanner.scan_directory(path)
    }

    /// Group music by album
    pub fn group_by_album(&self, music_list: &[MusicMetadata]) -> GroupedMusic {
        MusicScanner::group_by_album(music_list)
    }
}

impl Default for MusicService {
    fn default() -> Self {
        Self::new()
    }
}
