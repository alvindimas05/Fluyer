use rodio::Sink;
use serde::{Deserialize, Serialize};
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
                let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                    .expect("Failed to open default stream");
                GLOBAL_MUSIC_SINK
                    .set(rodio::Sink::connect_new(&stream_handle.mixer()))
                    .ok();
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
            GLOBAL_MUSIC_SINK.get().unwrap().clear();
            return;
        }

        if _command == MusicCommand::Next {
            GLOBAL_MUSIC_SINK.get().unwrap().skip_one();
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
    pub fn get_sync_info(is_from_next: bool) -> MusicPlayerSync {
        let sink = GLOBAL_MUSIC_SINK.get().unwrap();
        MusicPlayerSync {
            index: MUSIC_CURRENT_INDEX.load(Ordering::SeqCst),
            current_position: if is_from_next {
                0
            } else {
                sink.get_pos().as_millis()
            },
            is_playing: !sink.is_paused(),
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
        GLOBAL_MUSIC_SINK.get().unwrap().skip_one();
    }

    fn next() {
        let playlist = MUSIC_PLAYLIST.lock().unwrap();
        let current_idx = MUSIC_CURRENT_INDEX.load(Ordering::SeqCst);

        if let Some(music) = playlist.get(current_idx) {
            if let Ok(file) = File::open(music.path.clone()) {
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                GLOBAL_MUSIC_SINK.get().unwrap().append(source);

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
            let should_play_next = {
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
                sink.play();
            }

            for i in range {
                sink.set_volume(i as f32 / 20.);
                thread::sleep(Duration::from_millis(20));
            }
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
                let is_resuming = matches!(payload.value, WatcherStateType::Resume);

                if is_resuming {
                    app_handle
                        .emit(
                            crate::commands::route::MUSIC_PLAYER_SYNC,
                            MusicPlayer::get_sync_info(false),
                        )
                        .expect("Failed to sync music player");
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
                .ok();
        })
        .expect("Failed to watch headset change");
}
// Note: I have no idea what is this for but it's required for Rodio Android
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
