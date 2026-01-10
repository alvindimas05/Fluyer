#[cfg(mobile)]
use dotenvy_macro::dotenv;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tauri::Manager;
use tokio::process::Command;

use crate::state::app_handle;

static FFMPEG_PATH: OnceLock<PathBuf> = OnceLock::new();
static FFPROBE_PATH: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicMetadata {
    pub id: i16,
    pub path: String,
    pub duration: Option<u128>,
    pub filename: Option<String>,

    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track_number: Option<String>,
    pub genre: Option<String>,
    pub date: Option<String>,
    pub bits_per_sample: Option<u32>,
    pub sample_rate: Option<u32>,
    pub image: Option<String>,

    pub extra_tags: Option<HashMap<String, Option<String>>>,
}

#[cfg(mobile)]
const DEFAULT_TITLE: &str = dotenv!("VITE_DEFAULT_MUSIC_TITLE");
#[cfg(mobile)]
const DEFAULT_ARTIST: &str = dotenv!("VITE_DEFAULT_MUSIC_ARTIST");

impl MusicMetadata {
    #[cfg(mobile)]
    pub fn default_title() -> &'static str {
        DEFAULT_TITLE
    }
    #[cfg(mobile)]
    pub fn default_artist() -> &'static str {
        DEFAULT_ARTIST
    }
    pub fn artist_separator() -> &'static str {
        ";"
    }
    pub fn separator() -> &'static str {
        " • "
    }

    pub fn initialize_ffmpeg_paths() {
        let (ffmpeg_path, ffprobe_path) = {
            #[cfg(target_os = "linux")]
            {
                (
                    PathBuf::from("/usr/lib/fluyer/ffmpeg"),
                    PathBuf::from("/usr/lib/fluyer/ffprobe"),
                )
            }
            #[cfg(not(target_os = "linux"))]
            {
                (
                    app_handle()
                        .path()
                        .resource_dir()
                        .expect("Failed to get resource directory")
                        .join("libs/ffmpeg/bin/ffmpeg"),
                    app_handle()
                        .path()
                        .resource_dir()
                        .expect("Failed to get resource directory")
                        .join("libs/ffmpeg/bin/ffprobe"),
                )
            }
        };

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&ffmpeg_path, std::fs::Permissions::from_mode(0o755))
                .expect("Failed to set ffmpeg permissions");
            std::fs::set_permissions(&ffprobe_path, std::fs::Permissions::from_mode(0o755))
                .expect("Failed to set ffprobe permissions");
        }

        FFMPEG_PATH.set(ffmpeg_path).unwrap();
        FFPROBE_PATH.set(ffprobe_path).unwrap();
    }

    pub async fn get(path: String) -> Result<Self, String> {
        let output = Command::new(FFPROBE_PATH.get().unwrap())
            .args(&[
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                &path,
            ])
            .output()
            .await
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
        metadata.path = path.clone();

        // Extract format tags
        if let Some(format) = json.get("format") {
            if let Some(tags) = format.get("tags") {
                metadata.title = Self::extract_tag(tags, &["title", "TITLE", "Title"]);
                metadata.artist = Self::extract_tag(tags, &["artist", "ARTIST", "Artist"]);
                metadata.album_artist =
                    Self::extract_tag(tags, &["album_artist", "ALBUM_ARTIST", "ALBUMARTIST"]);
                metadata.album = Self::extract_tag(tags, &["album", "ALBUM", "Album"]);
                metadata.track_number = Self::extract_tag(tags, &["track", "TRACK", "TRACKNUMBER"]);
            }

            // Extract duration
            if let Some(duration_str) = format.get("duration").and_then(|v| v.as_str()) {
                if let Ok(duration_secs) = duration_str.parse::<f64>() {
                    metadata.duration = Some((duration_secs * 1000.0) as u128);
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
                            metadata.sample_rate = Some(rate as u32);
                        }
                    }

                    // Bit depth
                    if let Some(bits_per_sample) =
                        stream.get("bits_per_raw_sample").and_then(|v| v.as_str())
                    {
                        if let Ok(bits) = bits_per_sample.parse::<i32>() {
                            if bits > 0 {
                                metadata.bits_per_sample = Some(bits as u32);
                            }
                        }
                    } else if let Some(bits_per_sample) =
                        stream.get("bits_per_sample").and_then(|v| v.as_i64())
                    {
                        if bits_per_sample > 0 {
                            metadata.bits_per_sample = Some(bits_per_sample as u32);
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

    /// Extract cover art directly to memory as JPEG/PNG/BMP bytes
    pub async fn get_image_from_path(path: String) -> Result<Vec<u8>, String> {
        // First check if the file has any video stream (cover art)
        let probe_output = Command::new(FFPROBE_PATH.get().unwrap())
            .args(&[
                "-v",
                "quiet",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=codec_type",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                &path,
            ])
            .output()
            .await
            .map_err(|e| format!("Failed to probe for cover art: {}", e))?;

        // If no video stream found, return error early
        if probe_output.stdout.is_empty() || !probe_output.status.success() {
            return Err(format!("No cover art in file: {}", path));
        }

        // Try to copy the embedded image without re-encoding (fastest)
        let output = Command::new(FFMPEG_PATH.get().unwrap())
            .args(&[
                "-i",
                &path,
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
            .await
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if output.status.success() && !output.stdout.is_empty() {
            return Ok(output.stdout);
        }

        // If copy fails, try BMP encoder as fallback
        let output = Command::new(FFMPEG_PATH.get().unwrap())
            .args(&[
                "-i",
                &path,
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
            .await
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to extract cover art: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        if output.stdout.is_empty() {
            return Err(format!("No cover art in file: {}", path));
        }

        Ok(output.stdout)
    }

    fn get_artist_title_from_file_name(file_name: &str) -> Option<(String, String)> {
        let without_extension = file_name
            .rsplit_once('.')
            .map(|(name, _)| name)
            .unwrap_or(file_name);

        let patterns = vec![
            Regex::new(r"^(.*)\s-\s(.*)$").unwrap(),  // "Artist - Title"
            Regex::new(r"^(.*)\sby\s(.*)$").unwrap(), // "Title by Artist"
            Regex::new(r"^(.*)_(.*)$").unwrap(),      // "Artist_Title"
            Regex::new(r"^(.*)\s-\s(.*)\s\(.*\)$").unwrap(), // "Artist - Title (Remix)"
            Regex::new(r"^(.*)\s-\s(.*)\[.*\]$").unwrap(), // "Artist - Title [Remastered]"
        ];

        let cleanup_re = Regex::new(r"(?:\s+\([^)]*\)|\s+\[[^]]*\])$").unwrap();

        for pattern in patterns {
            if let Some(captures) = pattern.captures(without_extension) {
                let mut artist = captures
                    .get(1)
                    .map_or("", |m| m.as_str())
                    .trim()
                    .to_string();
                let mut title = captures
                    .get(2)
                    .map_or("", |m| m.as_str())
                    .trim()
                    .to_string();

                // Clean up both artist and title
                artist = cleanup_re.replace_all(&artist, "").trim().to_string();
                title = cleanup_re.replace_all(&title, "").trim().to_string();

                if pattern.as_str().contains("by") {
                    return Some((title, artist));
                }

                return Some((artist, title));
            }
        }

        None
    }

    pub fn get_lyrics_from_path(path: String) -> Option<String> {
        let lyrics_path = Path::new(&path).with_extension("lrc");
        if let Ok(lyrics) = std::fs::read_to_string(&lyrics_path) {
            return Some(lyrics);
        }
        None
    }
}
