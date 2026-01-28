pub mod decorum;
pub mod mobile;
pub mod playlist;
pub mod route;

pub const COMMAND_HANDLERS: fn(tauri::ipc::Invoke) -> bool = tauri::generate_handler![
    // Music commands
    crate::music::commands::music_controller,
    crate::music::commands::music_position_set,
    crate::folder::commands::music_get_all,
    crate::music::commands::music_playlist_add,
    crate::music::commands::music_playlist_remove,
    crate::music::commands::music_set_volume,
    crate::music::commands::music_playlist_goto,
    crate::music::commands::music_playlist_moveto,
    crate::music::commands::music_get_visualizer_buffer,
    crate::music::commands::music_get_image,
    // crate::music::commands::music_get_default_cover_art,
    crate::music::commands::music_get_current_duration,
    crate::music::commands::music_player_request_sync,
    crate::music::commands::music_get_lyrics,
    crate::music::commands::music_toggle_bit_perfect,
    #[cfg(desktop)]
    crate::music::commands::music_request_directory,
    #[cfg(desktop)]
    crate::music::commands::music_equalizer,
    #[cfg(desktop)]
    crate::music::commands::music_equalizer_reset,
    // Folder commands
    crate::folder::commands::folder_get_items,
    crate::folder::commands::folder_get_first_music_path,
    // System/Log commands
    crate::system::commands::log_error,
    crate::system::commands::log_info,
    #[cfg(target_os = "android")]
    crate::system::commands::toast,
    // Developer commands
    crate::system::commands::developer_save_log,
    crate::system::commands::developer_save_mpv_log,
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
    crate::coverart::commands::cover_art_get,
    // Lyric commands
    crate::lyric::commands::lyric_get,
    // Platform-specific commands
    #[cfg(windows)]
    decorum::decorum_show_snap_overlay,
    // Animated Background
    crate::animated_background::update_animated_background,
    crate::animated_background::restore_animated_background,
];
