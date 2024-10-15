use std::io::BufReader;

use rodio::Source;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::{MetadataOptions, StandardTagKey, Tag};
use symphonia::core::io::MediaSourceStream;
use base64::Engine;

#[derive(Debug)]
pub struct MusicMetadata {
    path: String,
    title: Option<String>,
    artist: Option<String>,
    album_artist: Option<String>,
    track_number: Option<String>,
    duration: Option<u128>,
    image: Option<String>
}

impl MusicMetadata {
    pub fn new(path: String) -> Self {
        MusicMetadata {
            path,
            title: None,
            artist: None,
            album_artist: None,
            track_number: None,
            duration: None,
            image: None
        }
    }
    
    pub fn get(&self) -> Self {
        let path = self.path.clone();
        let mut metadata = MusicMetadata::new(path.clone());
        
        let src = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                log::error!("Can't open music at path: {}", &path);
                return metadata;
            }
        };
    
        let mss = MediaSourceStream::new(Box::new(src.try_clone().unwrap()), Default::default());
    
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();
    
        let probed = match symphonia::default::get_probe().format(&Default::default(), mss, &fmt_opts, &meta_opts){
            Ok(probed) => probed,
            Err(_) => {
                log::error!("Unsupported format music: {}", &path);
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
                    if matches!(std_key, StandardTagKey::TrackTitle) {
                        metadata.title = self.get_value(tag);
                    }
                    if matches!(std_key, StandardTagKey::Artist) {
                        metadata.artist = self.get_value(tag);
                    }
                    if matches!(std_key, StandardTagKey::AlbumArtist) {
                        metadata.album_artist = self.get_value(tag);
                    }
                    if matches!(std_key, StandardTagKey::TrackNumber) {
                        metadata.track_number = self.get_value(tag);
                    }
                }
            }
            
            for visual in rev.visuals() {
                metadata.image = Some(base64::engine::general_purpose::STANDARD.encode(visual.data.clone()));
            }
        }
        
        let source = match rodio::Decoder::new(BufReader::new(src)){
            Ok(source) => source,
            Err(_) => {
                log::error!("Can't decode music: {}", &path);
                return metadata;
            }
        };
        if let Some(duration) = source.total_duration() {
            metadata.duration = Some(duration.as_millis())
        }
        
        metadata
    }
    
    fn get_value(&self, tag: &Tag) -> Option<String>{
        Some(tag.value.to_string())
    }
}

