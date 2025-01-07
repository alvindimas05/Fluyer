use std::path::Path;
use std::time::Duration;

use base64::Engine;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey, Tag};

#[derive(Debug, serde::Serialize, Clone)]
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
    pub image: Option<String>,
}

impl MusicMetadata {
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
            image: None,
        }
    }

    pub fn get(&self) -> Self {
        let path = self.path.clone();
        let mut metadata = MusicMetadata::new(path.clone());

        if let Some(filename) = Path::new(&path).file_stem() {
            metadata.filename = Some(filename.to_string_lossy().to_string());
        }
        let src = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Failed to open music at path: {}", &path);
                return metadata;
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
            Err(_) => {
                eprintln!("Unsupported format music: {}", &path);
                return metadata;
            }
        };

        let mut format = probed.format;

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

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
                        _ => {}
                    }
                }
            }

            for visual in rev.visuals() {
                metadata.image =
                    Some(base64::engine::general_purpose::STANDARD.encode(visual.data.clone()));
            }
        }
        
        if metadata.artist.is_none() {
            metadata.title = metadata.clone().filename;
        }

        let track = match format.default_track() {
            Some(track) => track,
            None => {
                eprintln!("Failed to find the track of music: {}", &path);
                return metadata;
            }
        };

        let sample_rate = match track.codec_params.sample_rate {
            Some(sample_rate) => sample_rate,
            None => {
                eprintln!("Unknown sample rate of music: {}", &path);
                return metadata;
            }
        };

        let total_frames = match track.codec_params.n_frames {
            Some(total_frames) => total_frames,
            None => {
                eprintln!("Unknown sample of frames of music: {}", &path);
                return metadata;
            }
        };

        metadata.duration =
            Some(Duration::from_secs_f64(total_frames as f64 / sample_rate as f64).as_millis());

        metadata
    }

    fn get_value(&self, tag: &Tag) -> Option<String> {
        Some(tag.value.to_string())
    }
}
