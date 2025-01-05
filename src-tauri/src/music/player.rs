use anyhow::Result;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, SendError, Sender};
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tauri::Emitter;
#[cfg(mobile)]
use tauri_plugin_fluyer::models::WatcherStateType;
use tauri_plugin_fluyer::FluyerExt;
// #[cfg(mobile)]
// use tauri_plugin_fluyer::FluyerExt;
use thread_priority::{ThreadBuilder, ThreadPriority};

use crate::GLOBAL_APP_HANDLE;

use super::metadata::MusicMetadata;

#[derive(Clone, Copy, Debug)]
pub enum MusicCommand {
    Pause,
    Play,
    Next,
    // WeakPause,
    // WeakPlay,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MusicState {
    Play,
    #[default]
    Pause,
}
// FIXME: Android Music Player requires restart app when audio jack is connected/disconnected
pub struct MusicPlayer {
    command: Sender<MusicCommand>,
    position: Sender<Duration>,
    playlist: Sender<Vec<String>>,
    info: Sender<()>,
}

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
}

// Bunch of music paths
static MUSIC_PLAYLIST: Mutex<Vec<MusicMetadata>> = Mutex::new(vec![]);
#[cfg(mobile)]
static MUSIC_IS_BACKGROUND: Mutex<bool> = Mutex::new(false);
#[cfg(mobile)]
static MUSIC_BACKGROUND_COUNT: Mutex<u8> = Mutex::new(0);
static MUSIC_STATE: Mutex<MusicState> = Mutex::new(MusicState::Play);

impl MusicPlayer {
    pub fn spawn() -> Self {
        let music_spawn = spawn();
        Self {
            command: music_spawn.0,
            position: music_spawn.1,
            playlist: music_spawn.2,
            info: music_spawn.3,
        }
    }
    pub fn play(&mut self) -> Result<(), SendError<MusicCommand>> {
        self.command.send(MusicCommand::Play)
    }
    pub fn pause(&mut self) -> Result<(), SendError<MusicCommand>> {
        self.command.send(MusicCommand::Pause)
    }
    pub fn next(&mut self) -> Result<(), SendError<MusicCommand>> {
        self.command.send(MusicCommand::Next)
    }
    pub fn set_pos(&mut self, position: u64) -> Result<(), SendError<Duration>> {
        self.position.send(Duration::from_millis(position))
    }
    pub fn get_info(&mut self) -> Result<(), SendError<()>> {
        self.info.send(())
    }
    pub fn add_playlist(&mut self, playlist: Vec<String>) -> Result<(), SendError<Vec<String>>> {
        self.playlist.send(playlist)
    }
}

#[cfg(mobile)]
pub fn handle_music_player_background() {
    GLOBAL_APP_HANDLE
        .get()
        .unwrap()
        .fluyer()
        .watch_state(|payload| {
            let is_resuming = matches!(payload.value, WatcherStateType::Resume);
            let mut state = MUSIC_IS_BACKGROUND.lock().unwrap();
            *state = !is_resuming;

            if is_resuming {
                let mut count = MUSIC_BACKGROUND_COUNT.lock().unwrap();
                GLOBAL_APP_HANDLE
                    .get()
                    .unwrap()
                    .emit(
                        crate::commands::route::MUSIC_PLAYER_SYNC,
                        MusicPlayerSync { skip: *count },
                    )
                    .expect("Failed to sync music player");
                *count = 0;
            }
        })
        .expect("Failed to watch app state");
}

#[cfg(mobile)]
pub fn handle_headset_change(// sender_sink_reset: Sender<bool>
) {
    GLOBAL_APP_HANDLE
        .get()
        .unwrap()
        .fluyer()
        .watch_headset_change(move |_payload| {
            // sender_sink_reset.send(payload.value).unwrap();
            // FIXME: Reset Sink after headset plugged/unplugged. Probably not possible but let's see...
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .emit(crate::commands::route::MUSIC_HEADSET_CHANGE, ())
                .unwrap_or_else(|_| {
                    eprintln!(
                        "Failed to emit {}",
                        crate::commands::route::MUSIC_HEADSET_CHANGE
                    )
                });
        })
        .expect("Failed to watch headset change");
}

fn spawn() -> (
    Sender<MusicCommand>,
    Sender<Duration>,
    Sender<Vec<String>>,
    Sender<()>,
) {
    let (sender_command, receiver_command) = unbounded();
    let (sender_position, receiver_position) = unbounded();
    let (sender_info, receiver_info) = unbounded();

    let (sender_next, receiver_next) = unbounded();
    let (sender_next_playlist, receiver_next_playlist) = unbounded();
    let (sender_next_position, receiver_next_position) = unbounded();
    let (sender_player_playlist, receiver_player_playlist) = unbounded();
    // let (sender_sink_reset, receiver_sink_reset) = unbounded();

    ThreadBuilder::default()
        .name("Music Player Next")
        .spawn_careless(move || {
            spawn_next_listener(
                receiver_next_playlist,
                receiver_next,
                receiver_next_position,
                sender_player_playlist,
            );
        })
        .expect("Failed to create music next thread");

    ThreadBuilder::default()
        .name("Music Player")
        .priority(ThreadPriority::Max)
        .spawn_careless(move || {
            play(
                receiver_command,
                receiver_position,
                receiver_player_playlist,
                receiver_info,
                // receiver_sink_reset,
                sender_next,
                sender_next_position,
            );
        })
        .expect("Failed to create music thread");
    (
        sender_command,
        sender_position,
        sender_next_playlist,
        sender_info,
    )
}

static MUSIC_NEXT_COUNTER: Mutex<i128> = Mutex::new(0);
fn spawn_next_listener(
    receiver_playlist: Receiver<Vec<String>>,
    receiver_next: Receiver<()>,
    receiver_next_position: Receiver<Duration>,
    sender_player_playlist: Sender<Vec<String>>,
) {
    let ms_countdown: u64 = 100;
    let mut counter = MUSIC_NEXT_COUNTER.lock().unwrap();
    let mut current_music_duration: u128 = 0;
    loop {
        if MUSIC_STATE.lock().unwrap().eq(&MusicState::Pause) {
            thread::sleep(Duration::from_millis(ms_countdown));
            continue;
        }

        let mut is_receiving_playlist = false;
        if let Ok(playlist) = receiver_playlist.try_recv() {
            is_receiving_playlist = true;
            for path in playlist.iter() {
                MUSIC_PLAYLIST
                    .lock()
                    .unwrap()
                    .push(MusicMetadata::new(path.clone()).get());
            }
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .emit(crate::commands::route::MUSIC_PLAYLIST_ADD, ())
                .unwrap_or_else(|_| {
                    eprintln!(
                        "Failed to emit {}",
                        crate::commands::route::MUSIC_PLAYLIST_ADD
                    )
                });
        }

        if let Ok(position) = receiver_next_position.try_recv() {
            *counter = (current_music_duration - position.as_millis()) as i128;
        }

        if receiver_next.try_recv().is_ok() {
            *counter = 0;
        }

        let mut playlist = MUSIC_PLAYLIST.lock().unwrap();
        if *counter <= 0 && !playlist.is_empty() {
            #[cfg(mobile)]
            {
                if *MUSIC_IS_BACKGROUND.lock().unwrap() {
                    let mut background_count = MUSIC_BACKGROUND_COUNT.lock().unwrap();
                    *background_count += 1;
                }
            }

            if !is_receiving_playlist {
                if playlist.len() > 1 {
                    GLOBAL_APP_HANDLE
                        .get()
                        .expect("Failed to get GLOBAL_APP_HANDLE")
                        .emit(crate::commands::route::MUSIC_PLAYER_NEXT, ())
                        .unwrap_or_else(|_| {
                            eprintln!(
                                "Failed to emit {}",
                                crate::commands::route::MUSIC_PLAYER_NEXT
                            )
                        });
                }
                playlist.remove(0);
            }
            if let Some(music) = playlist.first() {
                current_music_duration = music.duration.unwrap();
                sender_player_playlist
                    .send(vec![music.path.clone()])
                    .expect("Failed to send sender_player_playlist");
                *counter = music.duration.unwrap() as i128;
            }
        }
        if *counter > 0 {
            *counter -= ms_countdown as i128;
        }

        thread::sleep(Duration::from_millis(ms_countdown));
    }
}

#[allow(clippy::too_many_arguments)]
fn play(
    receiver_command: Receiver<MusicCommand>,
    receiver_position: Receiver<Duration>,
    receiver_player_playlist: Receiver<Vec<String>>,
    receiver_info: Receiver<()>,
    // receiver_sink_reset: Receiver<bool>,
    sender_next: Sender<()>,
    sender_next_position: Sender<Duration>,
) {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("Failed to open default stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    if MUSIC_STATE.lock().unwrap().eq(&MusicState::Pause) {
        sink.pause();
    }
    loop {
        // if let Ok(plugged) = receiver_sink_reset.try_recv() {
        //     if plugged {
        //         sink = Sink::connect_new(&stream_handle.mixer());
        //     }
        // }

        if let Ok(cmd) = receiver_command.try_recv() {
            match cmd {
                MusicCommand::Pause => {
                    // state = MusicState::Paused;
                    fade(&sink, true);
                }
                MusicCommand::Play => {
                    // state = MusicState::Playing;
                    fade(&sink, false);
                }
                MusicCommand::Next => {
                    sender_next.send(()).expect("Failed to send sender_next");
                    sink.skip_one();
                } // MusicCommand::WeakPause => fade(&sink, true),
                  // MusicCommand::WeakPlay => {
                  //     if state == MusicState::Playing {
                  //         fade(&sink, false);
                  //     }
                  // }
            }
        }

        if let Ok(playlist) = receiver_player_playlist.try_recv() {
            for path in playlist.iter() {
                sink_add_music(&sink, path.to_string());
            }
            // if let Some(app_handle) = GLOBAL_APP_HANDLE.get() {
            //     app_handle
            //         .emit(crate::commands::route::MUSIC_PLAYLIST_ADD, ())
            //         .expect("Can't emit music_playlist_add");
            // } else {
            //     eprintln!("Failed to get GLOBAL_APP_HANDLE");
            // }
        }

        if let Ok(position) = receiver_position.try_recv() {
            sender_next_position
                .send(position)
                .expect("Failed to send sender_next_position");
            if let Err(err) = sink.try_seek(position) {
                eprintln!("Failed to change position of music");
                eprintln!("{:#}", err);
            }
        }

        if receiver_info.try_recv().is_ok() {
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .emit(
                    crate::commands::route::MUSIC_GET_INFO,
                    get_music_player_info(&sink),
                )
                .unwrap_or_else(|_| {
                    eprintln!("Failed to emit {}", crate::commands::route::MUSIC_GET_INFO)
                });
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn sink_add_music(sink: &Sink, path: String) {
    if let Ok(file) = File::open(&path) {
        let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to read file");
        sink.append(source);
    } else {
        eprintln!("Failed to open file: {}", path);
    }
}

// fn sink_continue_latest_music(sink: &Sink) {
//     let playlist = MUSIC_PLAYLIST.lock().unwrap();
//     if playlist.is_empty() {
//         return;
//     }

//     sink_add_music(sink, playlist.first().unwrap().path.to_string());
//     sink.try_seek(Duration::from_millis(
//         *MUSIC_NEXT_COUNTER.lock().unwrap() as u64
//     ))
//     .expect("Failed to seek latest music");
// }

fn get_music_player_info(sink: &Sink) -> MusicPlayerInfo {
    MusicPlayerInfo {
        current_position: sink.get_pos().as_millis(),
        is_playing: !sink.is_paused(),
        music: MUSIC_PLAYLIST.lock().unwrap().first().cloned(),
    }
}

// fn fade(sink: &Sink, out: bool) {
//     if out {
//         sink.pause();
//     } else {
//         sink.play();
//     }
// }

fn fade(sink: &Sink, out: bool) {
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

    if out {
        *MUSIC_STATE.lock().unwrap() = MusicState::Pause;
        sink.pause();
    }

    if out {
        *MUSIC_STATE.lock().unwrap() = MusicState::Pause;
        sink.pause();
    } else {
        *MUSIC_STATE.lock().unwrap() = MusicState::Play;
        sink.play();
    }
}

// Note: I have no idea what is this for but it's required for Rodio Android
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
