use anyhow::{Error, Result};
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, SendError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum MusicCommand {
    Pause,
    Play,
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
    path: Sender<String>,
    position: Sender<Duration>
}
impl MusicPlayer {
    pub fn spawn() -> Self {
        let music_spawn = spawn(MusicState::Paused);
        Self {
            command: music_spawn.0,
            path: music_spawn.1,
            position: music_spawn.2
        }
    }
    pub fn play(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.command.send(MusicCommand::Play)
    }
    pub fn pause(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.command.send(MusicCommand::Pause)
    }
    pub fn set_path(&mut self, path: String) -> Result<(), SendError<String>>{
        self.path.send(path)
    }
    pub fn set_pos(&mut self, position: u64) -> Result<(), SendError<Duration>>{
        self.position.send(Duration::from_millis(position))
    }
}

fn spawn(initial_state: MusicState) -> (Sender<MusicCommand>, Sender<String>, Sender<Duration>) {
    let (sender_command, receiver_command) = mpsc::channel();
    let (sender_path, receiver_path) = mpsc::channel();
    let (sender_position, receiver_position) = mpsc::channel();
    thread::spawn(move || {
        if let Err(e) = play(&receiver_command, &receiver_path, &receiver_position, initial_state) {
            log::error!("Music thread crashed: {:#}", e);
        }
    });
    (sender_command, sender_path, sender_position)
}

fn play(
    receiver_command: &Receiver<MusicCommand>,
    receiver_path: &Receiver<String>,
    receiver_position: &Receiver<Duration>,
    initial_state: MusicState) -> Result<(), Error> {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default()?;

    let sink = Sink::try_new(&stream_handle)?;

    let mut state = initial_state;
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
                // MusicCommand::WeakPause => fade(&sink, true),
                // MusicCommand::WeakPlay => {
                //     if state == MusicState::Playing {
                //         fade(&sink, false);
                //     }
                // }
            }
        }
        let mut log_path: Option<String> = None;
        if let Ok(path) = receiver_path.try_recv() {
            log_path = Some(path.clone());
            if let Ok(file) = File::open(&path) {
                if state == MusicState::Playing && sink.empty() {
                    let source = rodio::Decoder::new(BufReader::new(file))?;
                    sink.append(source);
                }
            } else {
                log::error!("Failed to open file: {}", path);
            }
        }
        
        if let Ok(position) = receiver_position.try_recv() {
            if let Err(err) = sink.try_seek(position) {
                log::error!("Failed to change position of music: {}", log_path.unwrap());
                log::error!("{:#}", err);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

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