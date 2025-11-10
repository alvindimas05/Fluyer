use serde::{Deserialize, Serialize};
use tauri::Emitter;

#[cfg(desktop)]
use crate::logger;
use crate::music::metadata::MusicMetadata;
use crate::GLOBAL_APP_HANDLE;
#[cfg(desktop)]
use libmpv2::Mpv;
#[cfg(desktop)]
use std::sync::OnceLock;
use std::thread;
use libmpv2::Format;
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::models::PlaylistAddMusic;
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::models::{PlayerCommand, PlayerCommandArguments};
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::FluyerExt;

#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MusicCommand {
    #[default]
    None,
    Pause,
    Play,
    Next,
    Clear,
    Repeat,
    RepeatOne,
    RepeatNone,
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
    index: i64,
    current_position: Option<f64>,
    is_playing: bool,
}

const EQUALIZER_BANDS: [u32; 18] = [
    65, 92, 131, 185, 262, 370, 523, 740, 1047, 1480, 2093, 2960, 4186, 5920, 8372, 11840, 16744,
    20000,
];

#[cfg(desktop)]
pub static GLOBAL_MUSIC_MPV: OnceLock<Mpv> = OnceLock::new();

impl MusicPlayer {
    pub fn spawn() -> Self {
        Self::start_focus_listener();

        #[cfg(target_os = "linux")]
        unsafe {
            extern crate libc;
            use libc::{setlocale, LC_NUMERIC};
            use std::ffi::CString;
            setlocale(LC_NUMERIC, CString::new("C").unwrap().as_ptr());
        }

        #[cfg(desktop)]
        {
            let mpv = Mpv::with_initializer(|mpv| {
                let log_path = logger::get_mpv_log_path();
                mpv.set_option("log-file", log_path.clone())?;
                mpv.set_property("vo", "null")?;

                mpv.set_property("cache", "yes")?;
                mpv.set_property("demuxer-max-bytes", "100000000")?; // 100MB for high-res audio
                mpv.set_property("demuxer-readahead-secs", "10")?;

                mpv.set_property("prefetch-playlist", "yes")?;
                mpv.set_property("gapless-audio", "yes")?;

                mpv.set_property("af", "")?; // Clear audio filters
                mpv.set_property("audio-normalize-downmix", "no")?;
                mpv.set_property("audio-pitch-correction", "no")?;
                mpv.set_property("replaygain", "no")?; // Disable ReplayGain
                mpv.set_property("replaygain-fallback", "0")?;

                mpv.set_property("volume", "100")?;
                mpv.set_property("volume-max", "100")?;

                mpv.set_property("audio-samplerate", "0")?; // 0 = use source sample rate
                mpv.set_property("audio-channels", "auto")?; // Preserve channel layout

                #[cfg(target_os = "windows")]
                mpv.set_property("ao", "wasapi")?;

                #[cfg(target_os = "macos")]
                mpv.set_property("ao", "coreaudio")?;

                #[cfg(target_os = "linux")]{
                    mpv.set_property("ao", "alsa")?;
                    mpv.set_property("alsa-resample", "no")?; // Disable ALSA resampling
                }

                mpv.set_property("audio-buffer", "2")?;

                mpv.set_property("audio-stream-silence", "no")?;
                mpv.set_property("audio-wait-open", "2")?; // Wait for audio device

                mpv.set_property("msg-level", "all=warn,ao=debug")?;

                logger::debug!("The mpv log file is saved at {}", log_path);
                Ok(())
            }).unwrap();

            GLOBAL_MUSIC_MPV.set(mpv).ok();
        }
        Self::start_listener();

        Self {}
    }
    pub fn send_command(command: String) {
        let _command = match command.as_str() {
            "play" => MusicCommand::Play,
            "pause" | "stop" => MusicCommand::Pause,
            "next" => MusicCommand::Next,
            "clear" => MusicCommand::Clear,
            "repeat" => MusicCommand::Repeat,
            "repeatOne" => MusicCommand::RepeatOne,
            "repeatNone" => MusicCommand::RepeatNone,
            _ => MusicCommand::None,
        };
        if _command == MusicCommand::Play || _command == MusicCommand::Pause {
            Self::play_pause(_command == MusicCommand::Play);
            return;
        }

        if _command == MusicCommand::Clear {
            #[cfg(target_os = "android")]
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_run_command(PlayerCommandArguments::new(PlayerCommand::Clear))
                .ok();
            #[cfg(desktop)]
            {
                let mpv = GLOBAL_MUSIC_MPV.get().unwrap();
                mpv.command("playlist-clear", &[]).unwrap();
                if mpv.get_property::<i64>("playlist-pos").unwrap() >= 0 {
                    mpv.command("playlist-remove", &["0"]).unwrap();
                }
            }
            return;
        }

        if _command == MusicCommand::Next {
            #[cfg(target_os = "android")]
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_run_command(PlayerCommandArguments::new(PlayerCommand::Next))
                .ok();
            #[cfg(desktop)]
            GLOBAL_MUSIC_MPV
                .get()
                .unwrap()
                .command("playlist-next", &[])
                .unwrap();
            return;
        }

        if _command == MusicCommand::Repeat
            || _command == MusicCommand::RepeatOne
            || _command == MusicCommand::RepeatNone
        {
            #[cfg(target_os = "android")]
            {
                let args = PlayerCommandArguments::new(match _command {
                    MusicCommand::Repeat => PlayerCommand::Repeat,
                    MusicCommand::RepeatOne => PlayerCommand::RepeatOne,
                    MusicCommand::RepeatNone => PlayerCommand::RepeatNone,
                    _ => return,
                });
                GLOBAL_APP_HANDLE
                    .get()
                    .unwrap()
                    .fluyer()
                    .player_run_command(args)
                    .ok();
            }
            #[cfg(desktop)]
            {
                let mpv = GLOBAL_MUSIC_MPV.get().unwrap();
                mpv.set_property(
                    "loop-file",
                    if _command == MusicCommand::RepeatOne {
                        "inf"
                    } else {
                        "no"
                    },
                )
                .unwrap();
                mpv.set_property(
                    "loop-playlist",
                    if _command == MusicCommand::Repeat {
                        "inf"
                    } else {
                        "no"
                    },
                )
                .unwrap();
            }
            return;
        }
    }
    pub fn set_pos(position: u64) {
        #[cfg(target_os = "android")]
        {
            let mut args = PlayerCommandArguments::new(PlayerCommand::Seek);
            args.seek_position = Some(position);
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_run_command(args)
                .ok();
        }
        #[cfg(desktop)]
        GLOBAL_MUSIC_MPV
            .get()
            .unwrap()
            .command(
                "seek",
                &[
                    format!("{:.3}", position as f64 / 1000.0).as_str(),
                    "absolute",
                ],
            )
            .ok();
    }

    pub fn get_current_duration() -> f64 {
        #[cfg(target_os = "android")]
        {
            let info = GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_get_info()
                .unwrap();
            info.current_position
        }
        #[cfg(desktop)]
        {
            GLOBAL_MUSIC_MPV
                .get()
                .unwrap()
                .get_property::<f64>("time-pos")
                .unwrap() * 1000.0
        }
    }

    pub fn get_sync_info(is_reset: bool) -> MusicPlayerSync {
        #[cfg(target_os = "android")]
        {
            let info = GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_get_info()
                .unwrap();
            MusicPlayerSync {
                index: info.index,
                current_position: if is_reset {
                    0
                } else {
                    info.current_position
                },
                is_playing: if is_reset { true } else { info.is_playing },
            }
        }
        #[cfg(desktop)]
        {
            let mpv = GLOBAL_MUSIC_MPV.get().unwrap();
            let index = mpv.get_property::<i64>("playlist-pos").unwrap_or(-1);
            let time_pos = mpv.get_property::<f64>("time-pos");

            MusicPlayerSync {
                index: mpv.get_property::<i64>("playlist-pos").unwrap_or(-1),
                current_position: if is_reset {
                    Some(0.0)
                } else {
                    if let Ok(time) = time_pos {
                        Some(time * 1000.0)
                    } else {
                        None
                    }
                },
                is_playing: if is_reset {
                    true
                } else {
                    index > -1 && !mpv.get_property::<bool>("pause").unwrap()
                },
            }
        }
    }
    pub fn add_playlist(playlist: Vec<MusicMetadata>) {
        #[cfg(desktop)]
        {
            let mpv = GLOBAL_MUSIC_MPV.get().unwrap();

            for (_, music) in playlist.iter().enumerate() {
                // Note: libmpv2 v4.1.0
                // let path = format!("\"{}\"", music.path)
                //     .replace("\\", "/")
                //     .replace("//", "/");

                // Note: libmpv2 v5.0.0
                let path = format!("{}", music.path)
                    .replace("\\", "/")
                    .replace("//", "/");
                mpv.command("loadfile", &[path.as_str(), "append"]).unwrap();
            }
        }

        #[cfg(target_os = "android")]{
            let music_playlist = playlist
                .into_iter()
                .map(|music| PlaylistAddMusic {
                    file_path: music.path,
                    title: music.title.unwrap_or(MusicMetadata::default_title().to_string()),
                    artist: music.artist.unwrap_or(MusicMetadata::default_artist().to_string()),
                    // image: music.image,
                    image: None,
                })
                .collect::<Vec<_>>();
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_playlist_add(music_playlist)
                .unwrap();
        }
    }

    pub fn remove_playlist(index: usize) {
        #[cfg(target_os = "android")]
        {
            let mut args = PlayerCommandArguments::new(PlayerCommand::RemovePlaylist);
            args.playlist_remove_index = Some(index);
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_run_command(args)
                .unwrap();
        }

        #[cfg(desktop)]
        GLOBAL_MUSIC_MPV
            .get()
            .unwrap()
            .command("playlist-remove", &[index.to_string().as_str()])
            .unwrap();
    }

    pub fn goto_playlist(index: usize) {
        #[cfg(target_os = "android")]
        {
            let mut args = PlayerCommandArguments::new(PlayerCommand::GotoPlaylist);
            args.playlist_goto_index = Some(index);
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_run_command(args)
                .ok();
        }
        #[cfg(desktop)]
        GLOBAL_MUSIC_MPV
            .get()
            .unwrap()
            .command("playlist-play-index", &[index.to_string().as_str()])
            .unwrap();
    }

    pub fn moveto_playlist(from: usize, to: usize) {
        #[cfg(target_os = "android")]
        GLOBAL_APP_HANDLE
            .get()
            .unwrap()
            .fluyer()
            .player_playlist_move_to(from, to)
            .ok();
        #[cfg(desktop)]
        GLOBAL_MUSIC_MPV
            .get()
            .unwrap()
            .command(
                "playlist-move",
                &[from.to_string().as_str(), to.to_string().as_str()],
            )
            .unwrap();
    }

    pub fn set_volume(volume: f32) {
        #[cfg(target_os = "android")]
        {
            let mut args = PlayerCommandArguments::new(PlayerCommand::Volume);
            args.volume = Some(volume);
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .player_run_command(args)
                .ok();
        }
        #[cfg(desktop)]
        GLOBAL_MUSIC_MPV
            .get()
            .unwrap()
            .set_property("volume", (volume * 100.0).round() as i64)
            .unwrap();
    }

    fn play_pause(play: bool) {
        #[cfg(target_os = "android")]
        GLOBAL_APP_HANDLE
            .get()
            .unwrap()
            .fluyer()
            .player_run_command(PlayerCommandArguments::new(if play {
                PlayerCommand::Play
            } else {
                PlayerCommand::Pause
            }))
            .ok();
        #[cfg(desktop)]
        Self::mpv_play_pause(play);
    }

    pub fn equalizer(values: Vec<f32>) {
        #[cfg(desktop)]
        {
            let gains: String = values
                .iter()
                .map(|g| format!("{:.1}", g))
                .collect::<Vec<_>>()
                .join(" ");

            let bands: String = EQUALIZER_BANDS
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            let arg = format!(
                "lavfi=[anull[a];afireqsrc=preset=custom:gains={}:bands={}\
             [ir];[a][ir]afir]",
                gains, bands
            );

            GLOBAL_MUSIC_MPV
                .get()
                .unwrap()
                .command("af", &["set", &arg])
                .unwrap();
        }
    }

    pub fn reset_equalizer() {
        #[cfg(desktop)]
        {
            GLOBAL_MUSIC_MPV
                .get()
                .unwrap()
                .set_property("af", "")
                .unwrap();
        }
    }

    pub fn toggle_bit_perfect(enable: bool){
        #[cfg(desktop)]{
            GLOBAL_MUSIC_MPV.get().unwrap()
                .set_property("audio-exclusive", if enable { "yes" } else { "no" }).ok();
        }
    }

    #[cfg(desktop)]
    fn mpv_play_pause(play: bool) {
        let mpv = GLOBAL_MUSIC_MPV.get().unwrap();
        let playlist_count = mpv.get_property::<i64>("playlist-count").unwrap();

        if playlist_count < 1 {
            return;
        }
        if mpv.get_property::<i64>("playlist-pos").unwrap() < 0 {
            mpv.set_property("playlist-pos", 0).unwrap();
        }
        mpv.set_property("pause", !play).unwrap();

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

    pub fn request_sync(){
        Self::emit_sync(false);
    }

    pub fn emit_sync(is_reset: bool) {
        GLOBAL_APP_HANDLE
            .get()
            .unwrap()
            .emit(
                crate::commands::route::MUSIC_PLAYER_SYNC,
                Self::get_sync_info(is_reset),
            )
            .unwrap();
    }

    pub fn start_listener() {
        #[cfg(desktop)]
        {
            // Note: libmpv2 v4.1.0
            // use libmpv2::events::{Event, EventContext};
            // let mut event_context = EventContext::new(GLOBAL_MUSIC_MPV.get().unwrap().ctx);

            // Note: libmpv2 v5.0.0
            use libmpv2::events::Event;
            let mut client = GLOBAL_MUSIC_MPV.get().unwrap().create_client(None).unwrap();
            client.observe_property("time-pos", Format::Double, 1).unwrap();
            thread::spawn(move || loop {
                // let event = event_context.wait_event(0.1).unwrap_or(Err(libmpv2::Error::Null));
                let event = client.wait_event(0.1).unwrap_or(Err(libmpv2::Error::Null));
                match event {
                    Ok(Event::PropertyChange { name, .. }) => {
                        if name == "time-pos" {
                            Self::emit_sync(false);
                        }
                    }
                    Ok(Event::EndFile(reason)) => {
                        if reason == 0 {
                            Self::emit_sync(true);
                        }
                    }
                    Ok(_) => {},
                    Err(_) => {},
                }
            });
        }

        #[cfg(target_os = "android")]
        {
            GLOBAL_APP_HANDLE
                .get()
                .unwrap()
                .fluyer()
                .watch_playlist_change(|payload| {
                    // Note: Thread spawn is required for unknown reasons.
                    thread::spawn(move || {
                        println!("Music player background event: {:?}", payload);
                        Self::emit_sync(payload.is_next);
                    });
                })
                .unwrap();
        }
    }

    fn start_focus_listener() {
        use crate::GLOBAL_MAIN_WINDOW;
        use tauri::Listener;

        GLOBAL_MAIN_WINDOW
            .get()
            .unwrap()
            .listen("tauri://focus", move |_| {
                MusicPlayer::emit_sync(false);
            });
    }

}