use log::{info, warn};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct MusicMetadata {
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: String,
    pub bit_depth: String,
    pub sample_rate: String,
}

impl Default for MusicMetadata {
    fn default() -> Self {
        Self {
            file_path: String::new(),
            title: String::from("Unknown Title"),
            artist: String::from("Unknown Artist"),
            album: String::from("Unknown Album"),
            duration: String::from("0:00"),
            bit_depth: String::from("16-bit"),
            sample_rate: String::from("44.1 kHz"),
        }
    }
}

pub struct MusicScanner {
    ffmpeg_path: PathBuf,
    ffprobe_path: PathBuf,
}

impl MusicScanner {
    pub fn new() -> Self {
        // Use bundled FFmpeg
        let ffmpeg_path = PathBuf::from("libs/ffmpeg/bin/ffmpeg");
        let ffprobe_path = PathBuf::from("libs/ffmpeg/bin/ffprobe");

        // Ensure binaries are executable (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&ffmpeg_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&ffmpeg_path, perms);
            }
            if let Ok(metadata) = std::fs::metadata(&ffprobe_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&ffprobe_path, perms);
            }
        }

        Self {
            ffmpeg_path,
            ffprobe_path,
        }
    }

    /// Scan directory recursively for music files
    pub fn scan_directory(&self, path: &str) -> Vec<MusicMetadata> {
        use std::time::Instant;
        let start_time = Instant::now();

        info!("Starting music scan in: {}", path);

        // Collect all files (no extension filtering - let ffmpeg decide)
        let files: Vec<PathBuf> = WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file()
                    && e.path().file_name().unwrap_or_default() != "au_uu_SzH34yR2.mp3"
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        info!("Found {} files to scan", files.len());

        // Process files in parallel
        let results = Arc::new(Mutex::new(Vec::new()));
        let unsupported_count = Arc::new(Mutex::new(0usize));
        let chunk_size = (files.len() / num_cpus::get()).max(1);
        let mut handles = vec![];

        for chunk in files.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let results = Arc::clone(&results);
            let unsupported_count = Arc::clone(&unsupported_count);
            let ffprobe_path = self.ffprobe_path.clone();

            let handle = thread::spawn(move || {
                for file in chunk {
                    match Self::extract_metadata_static(&ffprobe_path, &file) {
                        Ok(metadata) => {
                            results.lock().unwrap().push(metadata);
                        }
                        Err(e) => {
                            // Log unsupported files at debug level to avoid spam
                            warn!(
                                "Unsupported or unreadable file: {:?} - {}",
                                file.display(),
                                e
                            );
                            *unsupported_count.lock().unwrap() += 1;
                        }
                    }
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        let mut all_metadata = results.lock().unwrap().clone();
        let unsupported = *unsupported_count.lock().unwrap();

        // Sort by artist, then album, then title
        all_metadata.sort_by(|a, b| {
            a.artist
                .cmp(&b.artist)
                .then(a.album.cmp(&b.album))
                .then(a.title.cmp(&b.title))
        });

        let elapsed = start_time.elapsed();
        info!(
            "Successfully processed {} music files ({} unsupported/unreadable) in {:.2}s",
            all_metadata.len(),
            unsupported,
            elapsed.as_secs_f64()
        );

        all_metadata
    }

    /// Extract metadata using ffprobe (static version for threading)
    fn extract_metadata_static(
        ffprobe_path: &Path,
        file_path: &Path,
    ) -> Result<MusicMetadata, String> {
        let output = Command::new(ffprobe_path)
            .args(&[
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                file_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

        if !output.status.success() {
            return Err(format!("ffprobe failed with status: {}", output.status));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let json: Value =
            serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Verify this is actually an audio file with at least one audio stream
        let has_audio_stream = json
            .get("streams")
            .and_then(|v| v.as_array())
            .map(|streams| {
                streams
                    .iter()
                    .any(|s| s.get("codec_type").and_then(|v| v.as_str()) == Some("audio"))
            })
            .unwrap_or(false);

        if !has_audio_stream {
            return Err("No audio stream found in file".to_string());
        }

        let mut metadata = MusicMetadata::default();
        metadata.file_path = file_path.to_string_lossy().to_string();

        // Extract format tags
        if let Some(format) = json.get("format") {
            if let Some(tags) = format.get("tags") {
                metadata.title = Self::extract_tag(tags, &["title", "TITLE", "Title"])
                    .unwrap_or_else(|| {
                        file_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("Unknown Title")
                            .to_string()
                    });

                metadata.artist = Self::extract_tag(
                    tags,
                    &[
                        "artist",
                        "ARTIST",
                        "Artist",
                        "album_artist",
                        "ALBUM_ARTIST",
                        "ALBUMARTIST",
                    ],
                )
                .map(|artist| Self::normalize_artist_separators(&artist))
                .unwrap_or_else(|| "Unknown Artist".to_string());

                metadata.album = Self::extract_tag(tags, &["album", "ALBUM", "Album"])
                    .unwrap_or_else(|| "Unknown Album".to_string());
            }

            // Extract duration
            if let Some(duration_str) = format.get("duration").and_then(|v| v.as_str()) {
                if let Ok(duration_secs) = duration_str.parse::<f64>() {
                    metadata.duration = Self::format_duration(duration_secs);
                }
            }
        }

        // Extract stream information (bit depth, sample rate)
        if let Some(streams) = json.get("streams").and_then(|v| v.as_array()) {
            for stream in streams {
                if stream.get("codec_type").and_then(|v| v.as_str()) == Some("audio") {
                    // Sample rate
                    if let Some(sample_rate) = stream.get("sample_rate").and_then(|v| v.as_str()) {
                        if let Ok(rate) = sample_rate.parse::<f64>() {
                            metadata.sample_rate = format!("{:.1} kHz", rate / 1000.0);
                        }
                    }

                    // Bit depth
                    if let Some(bits_per_sample) =
                        stream.get("bits_per_raw_sample").and_then(|v| v.as_str())
                    {
                        if let Ok(bits) = bits_per_sample.parse::<i32>() {
                            if bits > 0 {
                                metadata.bit_depth = format!("{}-bit", bits);
                            }
                        }
                    } else if let Some(bits_per_sample) =
                        stream.get("bits_per_sample").and_then(|v| v.as_i64())
                    {
                        if bits_per_sample > 0 {
                            metadata.bit_depth = format!("{}-bit", bits_per_sample);
                        }
                    }

                    break; // Use first audio stream
                }
            }
        }

        Ok(metadata)
    }

    /// Extract tag value from multiple possible keys (case-insensitive)
    fn extract_tag(tags: &Value, keys: &[&str]) -> Option<String> {
        // First try exact case-sensitive match
        for key in keys {
            if let Some(value) = tags.get(key).and_then(|v| v.as_str()) {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
        }

        // Then try case-insensitive match across all tag keys
        if let Some(obj) = tags.as_object() {
            for key in keys {
                let key_lower = key.to_lowercase();
                for (tag_key, tag_value) in obj {
                    if tag_key.to_lowercase() == key_lower {
                        if let Some(value) = tag_value.as_str() {
                            let trimmed = value.trim();
                            if !trimmed.is_empty() {
                                return Some(trimmed.to_string());
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Normalize artist separators - replace semicolons with commas
    fn normalize_artist_separators(artist: &str) -> String {
        artist
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join(", ")
    }

    /// Format duration in seconds to MM:SS or HH:MM:SS
    fn format_duration(seconds: f64) -> String {
        let total_secs = seconds as u64;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, secs)
        } else {
            format!("{}:{:02}", minutes, secs)
        }
    }

    /// Extract cover art directly to memory as JPEG/PNG/BMP bytes
    pub fn extract_cover_to_memory(&self, file_path: &str) -> Result<Vec<u8>, String> {
        // First check if the file has any video stream (cover art)
        let probe_output = Command::new(&self.ffprobe_path)
            .args(&[
                "-v",
                "quiet",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=codec_type",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                file_path,
            ])
            .output()
            .map_err(|e| format!("Failed to probe for cover art: {}", e))?;

        // If no video stream found, return error early
        if probe_output.stdout.is_empty() || !probe_output.status.success() {
            return Err("No cover art found in file".to_string());
        }

        // Try to copy the embedded image without re-encoding (fastest)
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i",
                file_path,
                "-an", // Disable audio
                "-c:v",
                "copy", // Copy video codec (don't re-encode)
                "-vframes",
                "1", // Extract only first frame
                "-f",
                "image2pipe", // Output to pipe
                "-",
            ])
            .output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if output.status.success() && !output.stdout.is_empty() {
            return Ok(output.stdout);
        }

        // If copy fails, try BMP encoder as fallback
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i",
                file_path,
                "-an", // Disable audio
                "-c:v",
                "bmp", // Use BMP encoder (built-in, no external deps)
                "-vframes",
                "1", // Extract only first frame
                "-f",
                "image2pipe", // Output to pipe
                "-",
            ])
            .output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "ffmpeg failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        if output.stdout.is_empty() {
            return Err("No cover art found in file".to_string());
        }

        Ok(output.stdout)
    }

    /// Extract cover art to a temporary file (optional enhancement)
    #[allow(dead_code)]
    pub fn extract_cover(&self, file_path: &str, output_path: &str) -> Result<(), String> {
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i",
                file_path,
                "-an", // Disable audio
                "-vcodec",
                "copy",
                "-y", // Overwrite output
                output_path,
            ])
            .output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "ffmpeg failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(MusicScanner::format_duration(45.5), "0:45");
        assert_eq!(MusicScanner::format_duration(125.0), "2:05");
        assert_eq!(MusicScanner::format_duration(3665.0), "1:01:05");
    }
}
