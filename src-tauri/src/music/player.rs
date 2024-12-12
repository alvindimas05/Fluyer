use anyhow::{Error, Result};
use rodio::{Sink, Source};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, SendError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
use thread_priority::{ThreadBuilder, ThreadPriority};

use crate::GLOBAL_APP_HANDLE;

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
    let (sender_playlist, receiver_playlist) = mpsc::channel();
    let (sender_info, receiver_info) = mpsc::channel();

    let _ = ThreadBuilder::default()
        .name("Music Player")
        .priority(ThreadPriority::Max)
        .spawn_careless(move || {
            if let Err(e) = play(
                initial_state,
                &receiver_command,
                &receiver_position,
                &receiver_playlist,
                &receiver_info,
            ) {
                log::error!("Music thread crashed: {:#}", e);
            }
        });
    (
        sender_command,
        sender_position,
        sender_playlist,
        sender_info,
    )
}

fn play(
    initial_state: MusicState,
    receiver_command: &Receiver<MusicCommand>,
    receiver_position: &Receiver<Duration>,
    receiver_playlist: &Receiver<Vec<String>>,
    receiver_info: &Receiver<()>,
) -> Result<(), Error> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    let sink = Sink::try_new(&stream_handle)?;
    let mut state = initial_state;

    if state == MusicState::Paused {
        sink.pause();
    }

    loop {
        if let Ok(cmd) = receiver_command.try_recv() {
            match cmd {
                MusicCommand::Pause => {
                    state = MusicState::Paused;
                    fade(&sink, true);
                }
                MusicCommand::Play => {
                    state = MusicState::Playing;
                    fade(&sink, false);
                }
                MusicCommand::Next => {
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
                if let Ok(file) = File::open(&path) {
                    let source = rodio::Decoder::new(BufReader::new(file))?;
                    sink.append(source);
                } else {
                    log::error!("Failed to open file: {}", path);
                }
            }
            if let Some(app_handle) = GLOBAL_APP_HANDLE.get() {
                app_handle
                    .emit("music_playlist_add", ())
                    .expect("Can't emit music_playlist_add");
            } else {
                log::error!("Failed to get GLOBAL_APP_HANDLE");
            }
        }

        if let Ok(position) = receiver_position.try_recv() {
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
