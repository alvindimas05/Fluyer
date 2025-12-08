use crate::music::player::MusicPlayer;

/// Send a control command to the music player
#[tauri::command]
pub fn music_controller(command: String) {
    MusicPlayer::send_command(command.clone());
}

/// Set the playback position
#[tauri::command]
pub fn music_position_set(position: u64) {
    MusicPlayer::set_pos(position);
}

/// Set the volume level
#[tauri::command]
pub fn music_set_volume(volume: f32) {
    MusicPlayer::set_volume(volume);
}

/// Get the current playback duration
#[tauri::command]
pub fn music_get_current_duration() -> Option<f64> {
    Some(MusicPlayer::get_current_duration())
}

/// Request player state sync
#[tauri::command]
pub fn music_player_request_sync() {
    MusicPlayer::request_sync();
}

/// Toggle bit-perfect audio mode
#[tauri::command]
pub fn music_toggle_bit_perfect(enable: bool) {
    MusicPlayer::toggle_bit_perfect(enable);
}

/// Set equalizer values (desktop only)
#[cfg(desktop)]
#[tauri::command]
pub fn music_equalizer(values: Vec<f32>) {
    MusicPlayer::equalizer(values);
}

/// Reset equalizer to default (desktop only)
#[cfg(desktop)]
#[tauri::command]
pub fn music_equalizer_reset() {
    MusicPlayer::reset_equalizer();
}
