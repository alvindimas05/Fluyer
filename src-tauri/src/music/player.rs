use serde::{Deserialize, Serialize};
use tauri::Emitter;
use thread_priority::ThreadBuilder;

#[cfg(target_os = "android")]
use std::thread;
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::models::{PlayerCommand, PlayerCommandArguments};
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::FluyerExt;
#[cfg(not(target_os = "android"))]
use std::sync::OnceLock;
#[cfg(not(target_os = "android"))]
use libmpv2::Mpv;

use crate::GLOBAL_APP_HANDLE;

#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MusicCommand {
    #[default]
    None,
    Pause,
    Play,
    Next,
    Clear,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MusicState {
    Play,
    #[default]
    Pause,
}
pub struct MusicPlayer {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayerSync {
    index: usize,
    current_position: u128,
    is_playing: bool,
}

#[cfg(not(target_os = "android"))]
pub static GLOBAL_MUSIC_MPV: OnceLock<Mpv> = OnceLock::new();

impl MusicPlayer {
    pub fn spawn() -> Self {
        handle_music_player_background();

        #[cfg(not(target_os = "android"))]{
            GLOBAL_MUSIC_MPV.set(Mpv::with_initializer(|mpv|{
                mpv.set_property("vo", "null")?;
                Ok(())
            }).unwrap()).ok();
            MusicPlayer::start_mpv_listener();
        }

        Self {}
    }
    pub fn send_command(&mut self, command: String) {
        let _command = match command.as_str() {
            "play" => MusicCommand::Play,
            "pause" | "stop" => MusicCommand::Pause,
            "next" => MusicCommand::Next,
            "clear" => MusicCommand::Clear,
            _ => MusicCommand::None,
        };
        if _command == MusicCommand::Play || _command == MusicCommand::Pause {
            self.play_pause(_command == MusicCommand::Play);
            return;
        }

        if _command == MusicCommand::Clear {
            #[cfg(target_os = "android")]
            GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_run_command(PlayerCommandArguments::new(PlayerCommand::RemovePlaylist)).ok();
            #[cfg(not(target_os = "android"))]{
                GLOBAL_MUSIC_MPV.get().unwrap().command("playlist-clear", &[]).unwrap();
                // GLOBAL_MUSIC_MPV.get().unwrap().command("playlist-remove", &["0"]).unwrap();
            }
            return;
        }

        if _command == MusicCommand::Next {
            #[cfg(target_os = "android")]
            GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_run_command(PlayerCommandArguments::new(PlayerCommand::Next)).ok();
            #[cfg(not(target_os = "android"))]
            GLOBAL_MUSIC_MPV.get().unwrap().command("playlist-next", &[]).unwrap();
            return;
        }
    }
    pub fn set_pos(&mut self, position: u64) {
        #[cfg(target_os = "android")] {
            let mut args = PlayerCommandArguments::new(PlayerCommand::Seek);
            args.seek_position = Some(position);
            GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_run_command(args).ok();
        } 
        #[cfg(not(target_os = "android"))] 
        GLOBAL_MUSIC_MPV.get().unwrap().command("seek", &[
            format!("{:.3}", position as f64 / 1000.0).as_str(), "absolute"]).unwrap();
    }
    pub fn get_sync_info(is_from_next: bool) -> MusicPlayerSync {
        // #[cfg(target_os = "android")] {
        //     let info = GLOBAL_APP_HANDLE.get().unwrap().fluyer()
        //         .player_get_info().unwrap();
        //     MusicPlayerSync {
        //         index,
        //         current_position: if is_from_next {
        //             0
        //         } else {
        //             info.current_position
        //         },
        //         is_playing: if is_from_next { true } else { info.is_playing },
        //     }
        // }
        #[cfg(not(target_os = "android"))] {
            let mpv = GLOBAL_MUSIC_MPV.get().unwrap();
            MusicPlayerSync {
                index: mpv.get_property::<i64>("playlist-pos").unwrap() as usize,
                current_position: if is_from_next {
                    0
                } else {
                    (mpv.get_property::<f64>("time-pos").unwrap() * 1000.0) as u128
                },
                is_playing: if is_from_next { true } else { !mpv.get_property::<bool>("pause").unwrap() },
            }
        }
    }
    pub fn add_playlist(&mut self, playlist: Vec<String>) {
        let mpv = GLOBAL_MUSIC_MPV.get().unwrap();
        let playlist_count = mpv.get_property::<i64>("playlist-count").unwrap();
        for (i, music) in playlist.iter().enumerate() {
            let path = format!("\"{}\"", music);
            mpv.command("loadfile", &[path.as_str(),
                if i < 1 && playlist_count < 1 { "replace" } else { "append" }]).unwrap();
        }
    }

    pub fn remove_playlist(&mut self, index: usize) {
        GLOBAL_MUSIC_MPV.get().unwrap().command("playlist-remove", &[index.to_string().as_str()]).unwrap();
    }

    pub fn goto_playlist(&mut self, index: usize) {
        #[cfg(target_os = "android")]
        GLOBAL_APP_HANDLE.get().unwrap().fluyer()
            .player_run_command(PlayerCommandArguments::new(PlayerCommand::Next)).ok();
        #[cfg(not(target_os = "android"))]
        GLOBAL_MUSIC_MPV.get().unwrap().command("playlist-play-index", &[index.to_string().as_str()]).unwrap();
    }

    pub fn set_volume(&mut self, volume: f32) {
        #[cfg(target_os = "android")] {
            let mut args = PlayerCommandArguments::new(PlayerCommand::Volume);
            args.volume = Some(volume);
            GLOBAL_APP_HANDLE.get().unwrap().fluyer().player_run_command(args).ok();
        }
        #[cfg(not(target_os = "android"))]{
            GLOBAL_MUSIC_MPV.get().unwrap().set_property("volume", (volume * 100.0).round() as i64).unwrap();
        }
    }
    
    fn play_pause(&self, play: bool){
        #[cfg(target_os = "android")]
        GLOBAL_APP_HANDLE.get().unwrap().fluyer()
            .player_run_command(PlayerCommandArguments::new(if play {
                PlayerCommand::Play
            } else {
                PlayerCommand::Pause
            })).ok();
        #[cfg(not(target_os = "android"))]
        self.mpv_play_pause(play);
    }

    #[cfg(not(target_os = "android"))]
    fn mpv_play_pause(&self, play: bool) {
        GLOBAL_MUSIC_MPV.get().unwrap().set_property("pause", !play).unwrap();
        
        // let sink = GLOBAL_MUSIC_SINK.get().unwrap();
        // let max_volume = MUSIC_VOLUME.load(Ordering::SeqCst);
        // let mut range: Vec<f32> = (if play { 1..21 } else { 0..20 })
        //     .map(|i| i as f32 * max_volume / 20.)
        //     .collect();
        // if play {
        //     sink.play();
        // } else {
        //     range.reverse();
        // }

        // for i in range {
        //     sink.set_volume(i);
        //     thread::sleep(Duration::from_millis(20));
        // }

        // if play {
        //     sink.play();
        // } else {
        //     sink.pause();
        // }
    }

    pub fn emit_sync(is_from_next: bool){
        if GLOBAL_MUSIC_MPV.get().unwrap().get_property::<f64>("playlist-pos").unwrap() < 0.0 {
            return
        }
        
        GLOBAL_APP_HANDLE.get().unwrap().emit(
            crate::commands::route::MUSIC_PLAYER_SYNC,
            MusicPlayer::get_sync_info(is_from_next),
        ).unwrap();
    }
    
    #[cfg(not(target_os = "android"))]
    pub fn start_mpv_listener(){
        use libmpv2::events::{EventContext, Event};

        let mut event_context = EventContext::new(GLOBAL_MUSIC_MPV.get().unwrap().ctx);
        ThreadBuilder::default().spawn_careless(move || loop {
            let event = event_context.wait_event(0.1).unwrap_or(Err(libmpv2::Error::Null));
            match event {
                Ok(Event::FileLoaded) => {
                    MusicPlayer::emit_sync(true);
                },
                Ok(_) => {},
                Err(_) => {},
            }
        }).unwrap();
    }
}

pub fn handle_music_player_background() {
    // Note: Disabled due to giving more issues than solving. Probably unnecessary though.
    #[cfg(not(target_os = "android"))]
    {
        use crate::GLOBAL_MAIN_WINDOW;
        use tauri::Listener;

        GLOBAL_MAIN_WINDOW
            .get()
            .unwrap()
            .listen("tauri://focus", move |_| {
                MusicPlayer::emit_sync(false);
            });
    }
    #[cfg(target_os = "android")]
    {
        use tauri_plugin_fluyer::models::WatcherStateType;
        use tauri_plugin_fluyer::FluyerExt;

        let app_handle = GLOBAL_APP_HANDLE.get().unwrap();
        app_handle
            .fluyer()
            .watch_state(move |payload| {
                if matches!(payload.value, WatcherStateType::Resume) && MUSIC_CURRENT_INDEX.load(Ordering::SeqCst) > 0 {
                    // Note: Probably can't be fixed. The app needs to be fully loaded somehow.
                    // Calling the emit_sync crashes the app without delay.
                    thread::spawn(||{
                        thread::sleep(Duration::from_millis(500));
                        MusicPlayer::emit_sync(false);
                    });
                }
            })
            .expect("Failed to watch app state");
    }
}