use anyhow::{Context, Result};
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
    music_command: Sender<MusicCommand>,
    music_path: Sender<String>
}
impl MusicPlayer {
    pub fn spawn() -> Self {
        let music_spawn = spawn(MusicState::Paused);
        Self {
            music_command: music_spawn.0,
            music_path: music_spawn.1
        }
    }
    pub fn play(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.music_command.send(MusicCommand::Play)
    }
    pub fn pause(&mut self) -> Result<(), SendError<MusicCommand>>{
        self.music_command.send(MusicCommand::Pause)
    }
    pub fn set_music_path(&mut self, path: String) -> Result<(), SendError<String>>{
        self.music_path.send(path)
    }
}

fn spawn(initial_state: MusicState) -> (Sender<MusicCommand>, Sender<String>) {
    let (sender_command, receiver_command) = mpsc::channel();
    let (sender_path, receiver_path) = mpsc::channel();
    thread::spawn(move || {
        if let Err(e) = play(&receiver_command, &receiver_path, initial_state) {
            println!("Music thread crashed: {:#}", e);
        }
    });
    (sender_command, sender_path)
}

fn play(receiver_command: &Receiver<MusicCommand>, receiver_path: &Receiver<String>, initial_state: MusicState) -> Result<()> {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().context("Failed to get output stream")?;

    let sink = Sink::try_new(&stream_handle).context("Failed to create Sink")?;

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

        if let Ok(path) = receiver_path.try_recv() {
            if let Ok(file) = File::open(&path) {
                if state == MusicState::Playing && sink.empty() {
                    println!("Playing music {}", &path);
                    let source = rodio::Decoder::new(BufReader::new(file))?;
                    sink.append(source);
                }
            } else {
                eprintln!("Failed to open file: {:?}", path);
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
}