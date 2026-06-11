use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn music_controller(state: State<AppState>, command: String) {
    state.music_player.send_command(command);
}

#[tauri::command]
pub fn music_position_set(state: State<AppState>, position: u64) {
    state.music_player.set_pos(position);
}

#[tauri::command]
pub fn music_volume_set(state: State<AppState>, volume: f32) {
    state.music_player.set_volume(volume);
}

#[tauri::command]
pub fn music_current_duration_get(state: State<AppState>) -> Option<f64> {
    Some(state.music_player.get_current_duration())
}

#[tauri::command]
pub fn music_player_request_sync(state: State<AppState>) {
    state.music_player.request_sync();
}

#[tauri::command]
pub fn music_bit_perfect_toggle(state: State<AppState>, enable: bool) {
    state.music_player.toggle_bit_perfect(enable);
}

#[cfg(desktop)]
#[tauri::command]
pub fn music_equalizer(state: State<AppState>, values: Vec<f32>) {
    state.music_player.equalizer(values);
}

#[cfg(desktop)]
#[tauri::command]
pub fn music_equalizer_reset(state: State<AppState>) {
    state.music_player.reset_equalizer();
}
