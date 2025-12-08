use crate::music::{metadata::MusicMetadata, player::MusicPlayer};

/// Add tracks to the playlist
#[tauri::command]
pub fn music_playlist_add(playlist: Vec<MusicMetadata>) {
    MusicPlayer::add_playlist(playlist);
}

/// Remove a track from the playlist by index
#[tauri::command]
pub fn music_playlist_remove(index: usize) {
    MusicPlayer::remove_playlist(index);
}

/// Jump to a specific track in the playlist
#[tauri::command]
pub fn music_playlist_goto(index: usize) {
    MusicPlayer::goto_playlist(index);
}

/// Move a track from one position to another in the playlist
#[tauri::command]
pub fn music_playlist_moveto(from: usize, to: usize) {
    MusicPlayer::moveto_playlist(from, to);
}
