pub mod coverart;
pub mod decorum;
pub mod developer;
pub mod folder;
pub mod log;
pub mod mobile;
pub mod music;
pub mod route;
pub mod playlist;

pub const COMMAND_HANDLERS: fn(tauri::ipc::Invoke) -> bool = tauri::generate_handler![
    // Music commands
    music::music_controller,
    music::music_position_set,
    music::music_get_all,
    music::music_playlist_add,
    music::music_playlist_remove,
    music::music_set_volume,
    music::music_playlist_goto,
    music::music_playlist_moveto,
    music::music_get_visualizer_buffer,
    music::music_get_image,
    music::music_get_current_duration,
    music::music_player_request_sync,
    music::music_get_lyrics,
    music::music_toggle_bit_perfect,
    #[cfg(desktop)]
    music::music_request_directory,
    #[cfg(desktop)]
    music::music_equalizer,
    #[cfg(desktop)]
    music::music_equalizer_reset,

    // Folder commands
    folder::folder_get_items,
    folder::folder_get_first_music_path,

    // Log commands
    log::log_error,
    log::log_info,
    #[cfg(target_os = "android")]
    log::toast,

    // Mobile commands
    #[cfg(target_os = "android")]
    mobile::request_read_audio_permission,
    #[cfg(mobile)]
    mobile::get_navigation_bar_height,
    #[cfg(mobile)]
    mobile::get_status_bar_height,
    #[cfg(mobile)]
    mobile::set_navigation_bar_visibility,
    #[cfg(target_os = "android")]
    mobile::android_request_directory,

    // Cover art commands
    coverart::cover_art_get,

    // Platform-specific commands
    #[cfg(windows)]
    decorum::decorum_show_snap_overlay,

    // Developer commands
    developer::developer_save_log,
    developer::developer_save_mpv_log,
];