use rodio::Sink;
use serde::{Deserialize, Serialize};
use tauri_plugin_fluyer::models::{PlayerCommand, PlayerCommandArguments};
use tauri_plugin_fluyer::FluyerExt;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
// #[cfg(windows)]
// use thread_priority::windows::WinAPIThreadPriority;
use thread_priority::{ThreadBuilder, ThreadPriority};

use crate::platform::is_android;
use crate::{platform, GLOBAL_APP_HANDLE};

use super::metadata::MusicMetadata;

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

pub static GLOBAL_MUSIC_SINK: OnceLock<Sink> = OnceLock::new();
static MUSIC_PLAYLIST: Mutex<Vec<MusicMetadata>> = Mutex::new(vec![]);
static MUSIC_CURRENT_INDEX: AtomicUsize = AtomicUsize::new(0);

impl MusicPlayer {
    pub fn spawn() -> Self {
        handle_music_player_background();

        ThreadBuilder::default()
            .name("Music Player")
            .priority(ThreadPriority::Max)
            .spawn(|_| {
                #[cfg(not(target_os = "android"))]{
                    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                        .expect("Failed to open default stream");
                    GLOBAL_MUSIC_SINK
                        .set(rodio::Sink::connect_new(&stream_handle.mixer()))
                        .ok();
                }
                MusicPlayer::start_playback_monitor();
            })
            .ok();

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
            self.play_pause(_command == MusicCommand::Pause);
            return;
        }

        if _command == MusicCommand::Clear {
            MUSIC_PLAYLIST.lock().unwrap().clear();
            MUSIC_CURRENT_INDEX.store(0, Ordering::SeqCst);
            if platform::is_android() {
                GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                    .player_run_command(PlayerCommandArguments::new(PlayerCommand::RemovePlaylist)).ok();
            } else {
                GLOBAL_MUSIC_SINK.get().unwrap().clear();
            }
            return;
        }

        if _command == MusicCommand::Next {
            if platform::is_android() {
                GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                    .player_run_command(PlayerCommandArguments::new(PlayerCommand::Next)).ok();
            } else {
                GLOBAL_MUSIC_SINK.get().unwrap().skip_one();
            }
            return;
        }
    }
    pub fn set_pos(&mut self, position: u64) {
        if platform::is_android() {
            let mut args = PlayerCommandArguments::new(PlayerCommand::Seek);
            args.seek_position = Some(position);
            GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_run_command(args).ok();
        } else {
            GLOBAL_MUSIC_SINK
                .get()
                .unwrap()
                .try_seek(Duration::from_millis(position))
                .ok();
        }
    }
    pub fn get_sync_info(is_from_next: bool) -> MusicPlayerSync {
        let index = MUSIC_CURRENT_INDEX.load(Ordering::SeqCst) - if is_from_next { 0 } else { 1 };
        if is_android() {
            let info = GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_get_info().unwrap();
            MusicPlayerSync {
                index,
                current_position: if is_from_next {
                    0
                } else {
                    info.current_position
                },
                is_playing: info.is_playing,
            }
        } else {
            let sink = GLOBAL_MUSIC_SINK.get().unwrap();
            MusicPlayerSync {
                index,
                current_position: if is_from_next {
                    0
                } else {
                    sink.get_pos().as_millis()
                },
                is_playing: !sink.is_paused(),
            }
        }
    }
    pub fn add_playlist(&mut self, playlist: Vec<String>) {
        MUSIC_PLAYLIST.lock().unwrap().append(
            &mut playlist
                .iter()
                .map(|path| MusicMetadata::new(path.clone()))
                .collect::<Vec<MusicMetadata>>(),
        );
    }

    pub fn remove_playlist(&mut self, index: usize) {
        let mut playlist = MUSIC_PLAYLIST.lock().unwrap();

        if let Some(_) = playlist.get(index) {
            if index > MUSIC_CURRENT_INDEX.load(Ordering::SeqCst) {
                MUSIC_CURRENT_INDEX.store(index + 1, Ordering::SeqCst);
            }
            playlist.remove(index);
        }
    }

    pub fn goto_playlist(&mut self, index: usize) {
        MUSIC_CURRENT_INDEX.store(index, Ordering::SeqCst);
        if is_android() {
            GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_run_command(PlayerCommandArguments::new(PlayerCommand::Next)).ok();
        } else {
            GLOBAL_MUSIC_SINK.get().unwrap().skip_one();
        }
    }

    fn next() {
        let playlist = MUSIC_PLAYLIST.lock().unwrap();
        let current_idx = MUSIC_CURRENT_INDEX.load(Ordering::SeqCst);

        if let Some(music) = playlist.get(current_idx) {
            if let Ok(file) = File::open(music.path.clone()) {
                if is_android() {
                    let mut args = PlayerCommandArguments::new(PlayerCommand::AddPlaylist);
                    args.playlist_file_path = Some(music.path.clone());
                    GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                        .player_run_command(args).ok();
                } else {
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    GLOBAL_MUSIC_SINK.get().unwrap().append(source);
                }
                
                GLOBAL_APP_HANDLE
                    .get()
                    .unwrap()
                    .emit(
                        crate::commands::route::MUSIC_PLAYER_SYNC,
                        MusicPlayer::get_sync_info(true),
                    )
                    .ok();
                MUSIC_CURRENT_INDEX.store(current_idx + 1, Ordering::SeqCst);
            }
        }
    }

    pub fn start_playback_monitor() {
        loop {
            let should_play_next = if is_android() {
                GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                    .player_is_empty().unwrap().value
            } else {
                let sink = GLOBAL_MUSIC_SINK.get().unwrap();
                sink.empty()
            };

            if should_play_next {
                MusicPlayer::next();
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        if is_android() {
            let mut args = PlayerCommandArguments::new(PlayerCommand::Volume);
            args.volume = Some(volume);
            GLOBAL_APP_HANDLE.get().unwrap().fluyer().player_run_command(args).ok();
        } else {
            GLOBAL_MUSIC_SINK.get().unwrap().set_volume(volume);
        }
    }
    
    fn play_pause(&self, pause: bool){
        if is_android() {
            GLOBAL_APP_HANDLE.get().unwrap().fluyer()
                .player_run_command(PlayerCommandArguments::new(if pause {
                    PlayerCommand::Pause
                } else {
                    PlayerCommand::Play
                })).ok();
        } else {
            self.sink_play_pause(pause);
        }
    }

    fn sink_play_pause(&self, out: bool) {
        let sink = GLOBAL_MUSIC_SINK.get().unwrap();
        let mut range: Vec<i32> = (1..20).collect();
        if out {
            range.reverse();
        } else {
            sink.play();
        }

        for i in range {
            sink.set_volume(i as f32 / 20.);
            thread::sleep(Duration::from_millis(20));
        }

        if out {
            sink.pause();
        } else {
            sink.play();
        }
    }
}

pub fn handle_music_player_background() {
    // Note: Disabled due to giving more issues than solving. Probably unnecessary though.
    // #[cfg(desktop)]
    // {
    //     use crate::GLOBAL_MAIN_WINDOW;
    //     use tauri::Listener;

    //     GLOBAL_MAIN_WINDOW
    //         .get()
    //         .unwrap()
    //         .listen("tauri://focus", move |_| {
    //             MUSIC_IS_BACKGROUND.store(false, Ordering::SeqCst);
    //             GLOBAL_APP_HANDLE
    //                 .get()
    //                 .unwrap()
    //                 .emit(
    //                     crate::commands::route::MUSIC_PLAYER_SYNC,
    //                     MusicPlayerSync {
    //                         skip: MUSIC_BACKGROUND_COUNT.load(Ordering::SeqCst),
    //                         info: MusicPlayer::get_info(),
    //                     },
    //                 )
    //                 .expect("Failed to sync music player");
    //             MUSIC_BACKGROUND_COUNT.store(0, Ordering::SeqCst);
    //         });

    //     GLOBAL_MAIN_WINDOW
    //         .get()
    //         .unwrap()
    //         .listen("tauri://blur", |_| {
    //             MUSIC_IS_BACKGROUND.store(true, Ordering::SeqCst);
    //         });
    // }
    #[cfg(target_os = "android")]
    {
        use tauri_plugin_fluyer::models::WatcherStateType;
        use tauri_plugin_fluyer::FluyerExt;

        let app_handle = GLOBAL_APP_HANDLE.get().unwrap();
        app_handle
            .fluyer()
            .watch_state(move |payload| {
                if matches!(payload.value, WatcherStateType::Resume) && MUSIC_CURRENT_INDEX.load(Ordering::SeqCst) > 0 {
                    // FIXME: Probably can't be fixed. The app needs to be fully loaded somehow.
                    // Calling the get_sync_info crashes the app without delay.
                    thread::spawn(||{
                        thread::sleep(Duration::from_millis(500));
                        app_handle
                            .emit(
                                crate::commands::route::MUSIC_PLAYER_SYNC,
                                MusicPlayer::get_sync_info(false),
                            )
                            .expect("Failed to sync music player");
                    });
                }
            })
            .expect("Failed to watch app state");
    }
}

// Note: I have no idea what is this for but it's required for Rodio Android
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
