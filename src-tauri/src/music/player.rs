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
    playlist: Sender<Vec<String>>
}
impl MusicPlayer {
    pub fn spawn() -> Self {
        let music_spawn = spawn(MusicState::Paused);
        Self {
            command: music_spawn.0,
            position: music_spawn.1,
            playlist: music_spawn.2
        }
    }
    pub fn play(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.command.send(MusicCommand::Play)
    }
    pub fn pause(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.command.send(MusicCommand::Pause)
    }
    pub fn next(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.command.send(MusicCommand::Next)
    }
    pub fn set_pos(&mut self, position: u64) -> Result<(), SendError<Duration>>{
        self.position.send(Duration::from_millis(position))
    }
    pub fn add_playlist(&mut self, playlist: Vec<String>) -> Result<(), SendError<Vec<String>>> {
        self.playlist.send(playlist)
    }
}

fn spawn(initial_state: MusicState) -> (Sender<MusicCommand>, Sender<Duration>, Sender<Vec<String>>) {
    let (sender_command, receiver_command) = mpsc::channel();
    let (sender_position, receiver_position) = mpsc::channel();
    let (sender_playlist, receiver_playlist) = mpsc::channel();
    
    thread::spawn(move || {
        if let Err(e) = play(&receiver_command, &receiver_position, &receiver_playlist, initial_state) {
            log::error!("Music thread crashed: {:#}", e);
        }
    });
    (sender_command, sender_position, sender_playlist)
}

fn play(
    receiver_command: &Receiver<MusicCommand>,
    receiver_position: &Receiver<Duration>,
    receiver_playlist: &Receiver<Vec<String>>,
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
                MusicCommand::Next => {
                    sink.skip_one();
                }
                // MusicCommand::WeakPause => fade(&sink, true),
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
                    if state == MusicState::Playing && sink.empty() {
                        let source = rodio::Decoder::new(BufReader::new(file))?;
                        sink.append(source);
                    }
                } else {
                    log::error!("Failed to open file: {}", path);
                }
            }
        }
        
        if let Ok(position) = receiver_position.try_recv() {
            if let Err(err) = sink.try_seek(position) {
                log::error!("Failed to change position of music");
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