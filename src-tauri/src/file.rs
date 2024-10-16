use walkdir::{DirEntry, WalkDir};
use crate::music::metadata::MusicMetadata;

fn is_audio_file(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension() {
        match ext.to_str().unwrap_or("").to_lowercase().as_str() {
            "mp3" | "flac" | "aac" => true,
            _ => false,
        }
    } else {
        false
    }
}

pub fn get_all_music() -> Vec<MusicMetadata> {
    let dir = "/Users/alvindimas05/Music/The Meaning Of Life";
    let mut musics: Vec<MusicMetadata> = vec![];
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file() && is_audio_file(e))
    {
        musics.push(MusicMetadata::new(
            String::from(entry.path().to_str().unwrap())
        ).get());
    }
    
    musics
}
