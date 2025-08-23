#[cfg(mobile)]
use dotenvy_macro::dotenv;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey, Tag};

use crate::logger;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MusicMetadata {
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

impl MusicMetadata {
    #[cfg(mobile)]
    pub fn default_title() -> String {
        dotenv!("VITE_DEFAULT_MUSIC_TITLE").to_string()
    }
    #[cfg(mobile)]
    pub fn default_artist() -> String {
        dotenv!("VITE_DEFAULT_MUSIC_ARTIST").to_string()
    }
    pub fn new(path: String) -> Self {
        MusicMetadata {
            path,
            duration: None,
            filename: None,
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            track_number: None,
            genre: None,
            date: None,
            bits_per_sample: None,
            sample_rate: None,
            image: None,
            extra_tags: Some(HashMap::new()),
        }
    }

    fn get_format(path: String) -> Option<Box<dyn FormatReader>> {
        let src = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                logger::error!("Failed to open music at path: {}", &path);
                return None;
            }
        };

        let mss = MediaSourceStream::new(Box::new(src.try_clone().unwrap()), Default::default());

        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = match symphonia::default::get_probe().format(
            &Default::default(),
            mss,
            &fmt_opts,
            &meta_opts,
        ) {
            Ok(probed) => probed,
            Err(e) => {
                logger::error!("Unsupported format music at {}: {:?}", &path, e);
                return None;
            }
        };
        let mut format = probed.format;
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }
        Some(format)
    }

    pub fn get(&self) -> Self {
        let path = self.path.clone();
        let mut metadata = MusicMetadata::new(path.clone());

        metadata.filename = Path::new(&path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let _format = MusicMetadata::get_format(path.clone());
        if _format.is_none() {
            return metadata;
        }

        let mut format = _format.unwrap();

        if let Some(rev) = format.metadata().current() {
            for tag in rev.tags() {
                if let Some(std_key) = tag.std_key {
                    match std_key {
                        StandardTagKey::TrackTitle => metadata.title = self.get_value(tag),
                        StandardTagKey::Artist => {
                            if metadata.artist.is_some()
                                && metadata.artist.clone().unwrap() != self.get_value(tag).unwrap()
                            {
                                metadata.artist = Some(format!(
                                    "{}||{}",
                                    metadata.artist.unwrap(),
                                    self.get_value(tag).unwrap()
                                ));
                            } else {
                                metadata.artist = self.get_value(tag);
                            }
                        }
                        StandardTagKey::Album => metadata.album = self.get_value(tag),
                        StandardTagKey::AlbumArtist => metadata.album_artist = self.get_value(tag),
                        StandardTagKey::TrackNumber => metadata.track_number = self.get_value(tag),
                        StandardTagKey::Genre => metadata.genre = self.get_value(tag),
                        StandardTagKey::Date => metadata.date = self.get_value(tag),
                        // key => {
                        //     let mut extra_tags = metadata.extra_tags.clone().unwrap();
                        //     extra_tags.insert(format!("{:?}", key), self.get_value(tag));
                        //     metadata.extra_tags = Some(extra_tags);
                        // }
                        _ => {}
                    }
                }
            }
        }

        if metadata.title.is_none() || metadata.artist.is_none() {
            if let Some((artist, title)) = MusicMetadata::get_artist_title_from_file_name(
                metadata.filename.clone().unwrap().as_str(),
            ) {
                if metadata.title.is_none() {
                    metadata.title = Some(title);
                }
                if metadata.artist.is_none() {
                    metadata.artist = Some(artist);
                }
            } else {
                metadata.title = metadata.filename.clone();
            }
        }

        let track = match format.default_track() {
            Some(track) => track,
            None => {
                logger::error!("Failed to find the track of music: {}", &path);
                return metadata;
            }
        };

        let time_base = match track.codec_params.time_base {
            Some(time_base) => Some(time_base),
            None => {
                logger::error!("Unknown sample of frames of music: {}", &path);
                None
            }
        };

        let n_frames = match track.codec_params.n_frames {
            Some(n_frames) => Some(n_frames),
            None => {
                logger::error!("Unknown sample of frames of music: {}", &path);
                None
            }
        };

        if let Some(timebase) = time_base {
            if let Some(nframes) = n_frames {
                metadata.duration = Some((timebase.calc_time(nframes).seconds * 1000) as u128);
            }
        }

        let bits_per_sample = match track.codec_params.bits_per_sample {
            Some(bits_per_sample) => bits_per_sample,
            None => {
                logger::error!("Unknown sample of frames of music: {}", &path);
                0
            }
        };
        metadata.bits_per_sample = Some(bits_per_sample);

        let sample_rate = match track.codec_params.sample_rate {
            Some(sample_rate) => sample_rate,
            None => {
                logger::error!("Unknown sample of frames of music: {}", &path);
                0
            }
        };
        metadata.sample_rate = Some(sample_rate);

        metadata
    }

    pub fn get_image_from_path(path: String) -> Option<String> {
        let _format = MusicMetadata::get_format(path);
        if _format.is_none() {
            return None;
        }
        let mut format = _format.unwrap();

        if let Some(rev) = format.metadata().current() {
            for visual in rev.visuals() {
                return Some(base64_simd::STANDARD.encode_to_string(visual.data.clone()));
            }
        }
        None
    }

    fn get_value(&self, tag: &Tag) -> Option<String> {
        Some(tag.value.to_string())
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
}
