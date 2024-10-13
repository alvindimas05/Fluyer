use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::{MetadataOptions, StandardTagKey, Tag};
use symphonia::core::io::MediaSourceStream;

#[derive(Debug)]
pub struct MusicMetadata {
    title: Option<String>,
    artist: Option<String>,
    album_artist: Option<String>,
    track_number: Option<String>
}

impl MusicMetadata {
    pub fn new() -> Self {
        MusicMetadata {
            title: None,
            artist: None,
            album_artist: None,
            track_number: None
        }
    }
    
    pub fn get(&self) -> Self {
        let src = std::fs::File::open("test-music.flac").expect("failed to open media");
    
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
    
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();
    
        let probed = symphonia::default::get_probe().format(&Default::default(), mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");
    
        let mut format = probed.format;
        
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }
        
        let mut metadata = MusicMetadata::new();
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
        }
        metadata
    }
    
    fn get_value(&self, tag: &Tag) -> Option<String>{
        Some(tag.value.to_string())
    }
}

