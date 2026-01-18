use walkdir::DirEntry;

/// Check if a directory entry is not hidden (doesn't start with '.')
pub fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with('.'))
        .unwrap_or(false)
}