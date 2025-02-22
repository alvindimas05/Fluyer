use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use tauri::Emitter;

use crate::{logger, platform, GLOBAL_APP_HANDLE};

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
    // WeakPause,
    // WeakPlay,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MusicState {
    Play,
    #[default]
    Pause,
}
pub struct MusicPlayer {}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayerInfo {
    current_position: u128,
    is_playing: bool,
    music: Option<MusicMetadata>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayerSync {
    skip: u8,
    info: MusicPlayerInfo,
}

pub static GLOBAL_MUSIC_SINK: OnceLock<Sink> = OnceLock::new();
// Bunch of music paths
static MUSIC_PLAYLIST: Mutex<Vec<MusicMetadata>> = Mutex::new(vec![]);
static MUSIC_IS_BACKGROUND: Mutex<bool> = Mutex::new(false);
static MUSIC_BACKGROUND_COUNT: Mutex<u8> = Mutex::new(0);
static MUSIC_STATE: Mutex<MusicState> = Mutex::new(MusicState::Play);
static MUSIC_CURRENT_INDEX: Mutex<usize> = Mutex::new(0);
static MUSIC_CURRENT_INDEX: Mutex<usize> = Mutex::new(0);

impl MusicPlayer {
    pub fn spawn() -> Self {
        // handle_music_player_background();

        thread::spawn(|| {
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("Failed to open default stream");
            GLOBAL_MUSIC_SINK
                .set(rodio::Sink::connect_new(&stream_handle.mixer()))
                .ok();

            let sink = GLOBAL_MUSIC_SINK.get().unwrap();
            if MUSIC_STATE.lock().unwrap().eq(&MusicState::Pause) {
                sink.pause();
            }
            MusicPlayer::start_playback_monitor();
            thread::sleep(Duration::from_secs(99999));
        });

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
            GLOBAL_MUSIC_SINK.get().unwrap().clear();
            return;
        }

        if _command == MusicCommand::Next {
            MusicPlayer::next();
            return;
        }
    }
    pub fn set_pos(&mut self, position: u64) {
        GLOBAL_MUSIC_SINK
            .get()
            .unwrap()
            .try_seek(Duration::from_millis(position))
            .ok();
    }
    pub fn get_info() -> MusicPlayerInfo {
        let sink = GLOBAL_MUSIC_SINK.get().unwrap();
        MusicPlayerInfo {
            current_position: sink.get_pos().as_millis(),
            is_playing: !sink.is_paused(),
            music: MUSIC_PLAYLIST.lock().unwrap().first().cloned(),
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
            playlist.remove(index);
        }
    }

    fn next() {
        let playlist = MUSIC_PLAYLIST.lock().unwrap();
        let current_idx = *MUSIC_CURRENT_INDEX.lock().unwrap();

        if let Some(music) = playlist.get(current_idx) {
            if let Ok(file) = File::open(music.path.clone()) {
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                GLOBAL_MUSIC_SINK.get().unwrap().append(source);

                if current_idx > 0 {
                    GLOBAL_APP_HANDLE
                        .get()
                        .unwrap()
                        .emit(crate::commands::route::MUSIC_PLAYER_NEXT, ())
                        .ok();
                }
                *MUSIC_CURRENT_INDEX.lock().unwrap() += 1;
                logger::debug!("Current music index is {}", MUSIC_CURRENT_INDEX.lock().unwrap());
            }
        }
    }

    pub fn start_playback_monitor() {
        thread::spawn(|| loop {
            let should_play_next = {
                let sink = GLOBAL_MUSIC_SINK.get().unwrap();
                sink.empty() && !sink.is_paused()
            };

            if should_play_next {
                MusicPlayer::next();
            }

            thread::sleep(Duration::from_millis(100));
        });
    }

    pub fn set_volume(&mut self, volume: f32) {
        GLOBAL_MUSIC_SINK.get().unwrap().set_volume(volume);
    }

    fn play_pause(&self, out: bool) {
        let sink = GLOBAL_MUSIC_SINK.get().unwrap();
        // Note : Due to delay issues on android. Disable smooth pause and play.
        if platform::is_desktop() {
            let mut range: Vec<i32> = (1..20).collect();
            if out {
                range.reverse();
            } else {
                *MUSIC_STATE.lock().unwrap() = MusicState::Play;
                sink.play();
            }

            for i in range {
                sink.set_volume(i as f32 / 20.);
                thread::sleep(Duration::from_millis(20));
            }
        }

        if out {
            *MUSIC_STATE.lock().unwrap() = MusicState::Pause;
            sink.pause();
        } else {
            *MUSIC_STATE.lock().unwrap() = MusicState::Play;
            sink.play();
        }
    }
}

pub fn handle_music_player_background() {
    #[cfg(desktop)]
    {
        use crate::GLOBAL_MAIN_WINDOW;
        use tauri::Listener;

        GLOBAL_MAIN_WINDOW
            .get()
            .unwrap()
            .listen("tauri://focus", move |_| {
                let mut state = MUSIC_IS_BACKGROUND.lock().unwrap();
                *state = false;
                let mut count = MUSIC_BACKGROUND_COUNT.lock().unwrap();
                GLOBAL_APP_HANDLE
                    .get()
                    .unwrap()
                    .emit(
                        crate::commands::route::MUSIC_PLAYER_SYNC,
                        MusicPlayerSync {
                            skip: *count,
                            info: MusicPlayer::get_info(),
                        },
                    )
                    .expect("Failed to sync music player");
                *count = 0;
            });

        GLOBAL_MAIN_WINDOW
            .get()
            .unwrap()
            .listen("tauri://blur", |_| {
                let mut state = MUSIC_IS_BACKGROUND.lock().unwrap();
                *state = true;
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
                let is_resuming = matches!(payload.value, WatcherStateType::Resume);
                let mut state = MUSIC_IS_BACKGROUND.lock().unwrap();
                *state = !is_resuming;

                if is_resuming {
                    let mut count = MUSIC_BACKGROUND_COUNT.lock().unwrap();
                    app_handle
                        .emit(
                            crate::commands::route::MUSIC_PLAYER_SYNC,
                            MusicPlayerSync {
                                skip: *count,
                                info: sink_get_player_info(),
                            },
                        )
                        .expect("Failed to sync music player");
                    sender_sync_info.send(()).unwrap();
                    *count = 0;
                }
            })
            .expect("Failed to watch app state");
    }
}

#[cfg(target_os = "android")]
pub fn handle_headset_change(/* sender_sink_reset: Sender<bool> */) {
    use tauri_plugin_fluyer::FluyerExt;
    GLOBAL_APP_HANDLE
        .get()
        .unwrap()
        .fluyer()
        .watch_headset_change(move |_payload| {
            // sender_sink_reset.send(payload.value).unwrap();
            // FIXME: Reset Sink after headset plugged/unplugged.
            // Note: Probably not possible but let's see...
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .emit(crate::commands::route::MUSIC_HEADSET_CHANGE, ())
                .unwrap_or_else(|_| {
                    logger::error!(
                        "Failed to emit {}",
                        crate::commands::route::MUSIC_HEADSET_CHANGE
                    )
                });
        })
        .expect("Failed to watch headset change");
}
// Note: I have no idea what is this for but it's required for Rodio Android
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
