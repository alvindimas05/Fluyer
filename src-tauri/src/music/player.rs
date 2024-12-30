use anyhow::{Error, Result};
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, SendError};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tauri::Emitter;
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
    #[default]
    Playing,
    Paused,
}
// FIXME: Android Music Player requires restart app when audio jack is connected/disconnected
pub struct MusicPlayer {
    command: Sender<MusicCommand>,
    position: Sender<Duration>,
    playlist: Sender<Vec<String>>,
    info: Sender<()>,
}

#[derive(Clone, Serialize)]
pub struct MusicPlayerInfo {
    current_position: u128,
    is_paused: bool,
}

// Bunch of music paths
static MUSIC_PLAYLIST: Mutex<Vec<MusicMetadata>> = Mutex::new(vec![]);

impl MusicPlayer {
    pub fn spawn() -> Self {
        let music_spawn = spawn(MusicState::Paused);
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

fn spawn(
    initial_state: MusicState,
) -> (
    Sender<MusicCommand>,
    Sender<Duration>,
    Sender<Vec<String>>,
    Sender<()>,
) {
    let (sender_command, receiver_command) = mpsc::channel();
    let (sender_position, receiver_position) = mpsc::channel();
    let (sender_info, receiver_info) = mpsc::channel();

    let (sender_next, receiver_next) = mpsc::channel();
    let (sender_next_playlist, receiver_next_playlist) = mpsc::channel();
    let (sender_next_position, receiver_next_position) = mpsc::channel();
    let (sender_player_playlist, receiver_player_playlist) = mpsc::channel();

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
            if let Err(e) = play(
                initial_state,
                receiver_command,
                receiver_position,
                receiver_player_playlist,
                receiver_info,
                sender_next,
                sender_next_position,
            ) {
                log::error!("Music thread crashed: {:#}", e);
            }
        })
        .expect("Failed to create music thread");
    (
        sender_command,
        sender_position,
        sender_next_playlist,
        sender_info,
    )
}

fn spawn_next_listener(
    receiver_playlist: Receiver<Vec<String>>,
    receiver_next: Receiver<()>,
    receiver_next_position: Receiver<Duration>,
    sender_player_playlist: Sender<Vec<String>>,
) {
    let ms_countdown: u64 = 100;
    let mut counter: i128 = 0;
    let mut current_music_duration: u128 = 0;
    loop {
        let mut is_receiving_playlist = false;
        if let Ok(playlist) = receiver_playlist.try_recv() {
            is_receiving_playlist = true;
            for path in playlist.iter() {
                MUSIC_PLAYLIST
                    .lock()
                    .unwrap()
                    .push(MusicMetadata::new(path.clone()).get());
            }
            if let Some(app_handle) = GLOBAL_APP_HANDLE.get() {
                app_handle
                    .emit("music_playlist_add", ())
                    .expect("Can't emit music_playlist_add");
            } else {
                log::error!("Failed to get GLOBAL_APP_HANDLE");
            }
        }

        if let Ok(position) = receiver_next_position.try_recv() {
            counter = (current_music_duration - position.as_millis()) as i128;
        }

        let mut playlist = MUSIC_PLAYLIST.lock().unwrap();
        if counter <= 0 && !playlist.is_empty() {
            if !is_receiving_playlist {
                if playlist.len() > 1 {
                    GLOBAL_APP_HANDLE
                        .get()
                        .expect("Failed to get GLOBAL_APP_HANDLE")
                        .emit("music_player_next", ())
                        .expect("Failed to emit music_player_next");
                }
                playlist.remove(0);
            }
            if let Some(music) = playlist.first() {
                current_music_duration = music.duration.unwrap();
                sender_player_playlist
                    .send(vec![music.path.clone()])
                    .expect("Failed to send sender_player_playlist");
                counter = music.duration.unwrap() as i128;
            }
        }
        if counter > 0 {
            counter -= ms_countdown as i128;
        }

        thread::sleep(Duration::from_millis(ms_countdown));
    }
}

fn play(
    initial_state: MusicState,
    receiver_command: Receiver<MusicCommand>,
    receiver_position: Receiver<Duration>,
    receiver_playlist: Receiver<Vec<String>>,
    receiver_info: Receiver<()>,
    sender_next: Sender<()>,
    sender_next_position: Sender<Duration>,
) -> Result<(), Error> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    let sink = Sink::try_new(&stream_handle)?;
    // let mut state = initial_state;

    if initial_state == MusicState::Paused {
        sink.pause();
    }

    loop {
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

        if let Ok(playlist) = receiver_playlist.try_recv() {
            for path in playlist.iter() {
                if let Ok(file) = File::open(path) {
                    let source = rodio::Decoder::new(BufReader::new(file))?;
                    sink.append(source);
                } else {
                    log::error!("Failed to open file: {}", path);
                }
            }
            // if let Some(app_handle) = GLOBAL_APP_HANDLE.get() {
            //     app_handle
            //         .emit("music_playlist_add", ())
            //         .expect("Can't emit music_playlist_add");
            // } else {
            //     log::error!("Failed to get GLOBAL_APP_HANDLE");
            // }
        }

        if let Ok(position) = receiver_position.try_recv() {
            sender_next_position
                .send(position)
                .expect("Failed to send sender_next_position");
            if let Err(err) = sink.try_seek(position) {
                log::error!("Failed to change position of music");
                log::error!("{:#}", err);
            }
        }

        if let Ok(_info) = receiver_info.try_recv() {
            if let Some(app_handle) = GLOBAL_APP_HANDLE.get() {
                app_handle
                    .emit(
                        "music_get_info",
                        MusicPlayerInfo {
                            current_position: sink.get_pos().as_millis(),
                            is_paused: sink.is_paused(),
                        },
                    )
                    .expect("Can't emit music_get_info");
            } else {
                log::error!("Failed to get GLOBAL_APP_HANDLE");
            }
        }

        thread::sleep(Duration::from_millis(100));
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
        sink.play();
    }

    for i in range {
        sink.set_volume(i as f32 / 20.);
        thread::sleep(Duration::from_millis(20));
    }

    if out {
        sink.pause();
    }

    if out {
        sink.pause();
    } else {
        sink.play();
    }
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
