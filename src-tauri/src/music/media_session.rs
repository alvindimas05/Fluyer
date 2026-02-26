#![allow(unused_variables)]

use crate::music::metadata::MusicMetadata;
use crate::state::app_handle;

#[cfg(target_os = "android")]
use tauri_plugin_fluyer::FluyerExt;

pub struct MediaSession;

impl MediaSession {
    pub fn init() {
        #[cfg(target_os = "android")]
        {
            crate::info!("Initializing Android Media Control");
            let _ = app_handle().fluyer().init_media_control(|event| {
                crate::info!("Media Control Action: {}", event.action);
                if event.action.starts_with("seek:") {
                    if let Ok(pos) = event.action[5..].parse::<u64>() {
                        std::thread::spawn(move || {
                            crate::music::player::MusicPlayer::set_pos(pos);
                        });
                    }
                } else if event.action == "previous" {
                    std::thread::spawn(move || {
                        crate::music::player::MusicPlayer::play_previous();
                    });
                } else if event.action == "next" {
                    std::thread::spawn(move || {
                        crate::music::player::MusicPlayer::play_next(true);
                    });
                } else {
                    std::thread::spawn(move || {
                        crate::music::player::MusicPlayer::send_command(event.action);
                    });
                }
            });
        }

        #[cfg(desktop)]
        {
            // Desktop media session initialization to be implemented later
        }
    }

    pub fn set_state(is_playing: bool, position: u64) {
        #[cfg(target_os = "android")]
        {
            let _ = app_handle()
                .fluyer()
                .set_media_control_state(is_playing, position);
        }

        #[cfg(desktop)]
        {
            // Desktop set_state to be implemented later
        }
    }

    pub fn update_metadata(music: &MusicMetadata, is_playing: bool, is_first: bool, is_last: bool) {
        #[cfg(target_os = "android")]
        {
            let handle = app_handle();
            let image_path = match handle.fluyer().metadata_get_image(music.path.clone()) {
                Ok(res) => res.path,
                Err(_) => None,
            };

            let _ = handle.fluyer().update_media_control(
                music.title.clone().unwrap_or("Unknown".to_string()),
                music.artist.clone().unwrap_or("Unknown".to_string()),
                music.album.clone().unwrap_or("Unknown".to_string()),
                music.duration.unwrap_or(0) as u64,
                image_path,
                is_playing,
                is_first,
                is_last,
            );
        }

        #[cfg(desktop)]
        {
            // Desktop update_metadata to be implemented later
        }
    }
}
