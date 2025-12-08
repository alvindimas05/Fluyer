use walkdir::DirEntry;

/// Check if a file is a supported audio format
pub fn is_supported_audio_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "mp3" | "flac" | "aac" | "m4a" | "wav" | "ogg" | "alac" | "opus"
        )
    } else {
        false
    }
}

/// Check if a directory entry is not hidden (doesn't start with '.')
pub fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with('.'))
        .unwrap_or(false)
}

/// Normalize Windows paths by fixing double backslashes
pub fn normalize_path(path: String) -> String {
    path.replace(":\\\\", ":\\")
}
