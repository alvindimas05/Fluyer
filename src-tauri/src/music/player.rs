use std::ffi::CString;
use std::ptr;
use std::sync::{OnceLock, Mutex};
use std::thread;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use crate::{logger, GLOBAL_APP_HANDLE};
use crate::music::metadata::MusicMetadata;

#[cfg(target_os = "android")]
use tauri_plugin_fluyer::models::PlaylistAddMusic;
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::models::{PlayerCommand, PlayerCommandArguments};
#[cfg(target_os = "android")]
use tauri_plugin_fluyer::FluyerExt;

const BASS_PLUGINS: [&str; 2] = ["bassflac", "bassmix"];

const BASS_SAMPLE_FLOAT: u32 = 0x100;
const BASS_STREAM_DECODE: u32 = 0x200000;
const BASS_MIXER_NORAMPIN: u32 = 0x800000;
const BASS_ACTIVE_STOPPED: u32 = 0;
const BASS_ACTIVE_PLAYING: u32 = 1;
const BASS_ACTIVE_PAUSED: u32 = 3;
const BASS_POS_BYTE: u32 = 0;
const BASS_ATTRIB_VOL: u32 = 2;

#[cfg(desktop)]
#[link(name = "bass")]
#[link(name = "bassmix")]
extern "C" {
    fn BASS_Init(device: i32, freq: u32, flags: u32, win: *mut std::ffi::c_void, clsid: *mut std::ffi::c_void) -> i32;
    fn BASS_PluginLoad(file: *const std::ffi::c_char, flags: u32) -> u32;
    fn BASS_PluginFree(handle: u32) -> i32;
    fn BASS_StreamCreateFile(mem: bool, file: *const std::ffi::c_void, offset: u64, length: u64, flags: u32) -> u32;
    fn BASS_Mixer_StreamCreate(freq: u32, chans: u32, flags: u32) -> u32;
    fn BASS_Mixer_StreamAddChannel(handle: u32, channel: u32, flags: u32) -> i32;
    fn BASS_Mixer_ChannelRemove(handle: u32) -> u32;
    fn BASS_Mixer_ChannelIsActive(handle: u32) -> u32;
    fn BASS_StreamFree(handle: u32) -> i32;
    fn BASS_ChannelPlay(handle: u32, restart: i32) -> i32;
    fn BASS_ChannelPause(handle: u32) -> i32;
    fn BASS_ChannelStop(handle: u32) -> i32;
    fn BASS_ChannelIsActive(handle: u32) -> u32;
    fn BASS_ChannelGetLength(handle: u32, mode: u32) -> u64;
    fn BASS_ChannelGetPosition(handle: u32, mode: u32) -> u64;
    fn BASS_ChannelSetPosition(handle: u32, pos: u64, mode: u32) -> i32;
    fn BASS_ChannelBytes2Seconds(handle: u32, pos: u64) -> f64;
    fn BASS_ChannelSeconds2Bytes(handle: u32, pos: f64) -> u64;
    fn BASS_ChannelSetAttribute(handle: u32, attrib: u32, value: f32) -> i32;
    fn BASS_ChannelGetAttribute(handle: u32, attrib: u32, value: *mut f32) -> i32;
    fn BASS_ErrorGetCode() -> i32;
    fn BASS_Free() -> i32;
}

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

#[derive(Clone, Debug)]
struct PlaylistItem {
    metadata: MusicMetadata,
}

pub struct MusicPlayer {}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RepeatMode {
    None,
    All,
    One,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayerSync {
    index: i64,
    current_position: Option<f64>,
    is_playing: bool,
}

#[cfg(desktop)]
static mut GLOBAL_BASS_MIXER: u32 = 0;
#[cfg(desktop)]
static mut CURRENT_STREAM: u32 = 0;
#[cfg(desktop)]
static GLOBAL_PLAYER_STATE: OnceLock<Mutex<PlayerState>> = OnceLock::new();

#[cfg(desktop)]
#[derive(Debug, Clone)]
struct PlayerState {
    playlist: Vec<PlaylistItem>,
    current_index: Option<usize>,
    repeat_mode: RepeatMode,
}

impl MusicPlayer {
    pub fn spawn() -> Self {
        Self::start_focus_listener();

        #[cfg(desktop)]
        {
            // Initialize global player state
            GLOBAL_PLAYER_STATE.get_or_init(|| {
                Mutex::new(PlayerState {
                    playlist: Vec::new(),
                    current_index: None,
                    repeat_mode: RepeatMode::None,
                })
            });

            unsafe {
                if BASS_Init(-1, 44100, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
                    logger::error!("Failed to initialize BASS, error: {}", BASS_ErrorGetCode());
                } else {
                    logger::info!("BASS initialized successfully");
                }

                // Load plugins based on platform
                #[cfg(target_os = "macos")]
                let extension = "dylib";
                #[cfg(target_os = "windows")]
                let extension = "dll";
                #[cfg(target_os = "linux")]
                let extension = "so";

                for plugin in BASS_PLUGINS {
                    let c_path = CString::new(format!("{}.{}", plugin, extension)).unwrap();
                    let handle = BASS_PluginLoad(c_path.as_ptr(), 0);

                    if handle == 0 {
                        logger::warn!("Failed to load plugin: {}, error: {}", plugin, BASS_ErrorGetCode());
                    } else {
                        logger::info!("Loaded plugin: {}", plugin);
                    }
                }

                GLOBAL_BASS_MIXER = BASS_Mixer_StreamCreate(44100, 2, BASS_SAMPLE_FLOAT);
                if GLOBAL_BASS_MIXER == 0 {
                    logger::error!("Failed to create BASS mixer stream, error: {}", BASS_ErrorGetCode());
                } else {
                    logger::info!("BASS mixer created successfully");
                }
            }
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
            Self::clear_playlist();
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
            Self::play_next();
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
                if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
                    if let Ok(mut state) = state_lock.lock() {
                        state.repeat_mode = match _command {
                            MusicCommand::Repeat => RepeatMode::All,
                            MusicCommand::RepeatOne => RepeatMode::One,
                            MusicCommand::RepeatNone => RepeatMode::None,
                            _ => RepeatMode::None,
                        };
                    }
                }
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
        unsafe {
            if CURRENT_STREAM != 0 && GLOBAL_BASS_MIXER != 0 {
                // Pause the mixer to prevent delay from buffered data
                BASS_ChannelPause(GLOBAL_BASS_MIXER);

                // Set the new position on the source stream
                let seconds = position as f64 / 1000.0;
                let byte_pos = BASS_ChannelSeconds2Bytes(CURRENT_STREAM, seconds);
                if BASS_ChannelSetPosition(CURRENT_STREAM, byte_pos, BASS_POS_BYTE) == 0 {
                    logger::error!("Failed to set position, error: {}", BASS_ErrorGetCode());
                }

                // Restart the mixer with restart=true to clear the buffer
                BASS_ChannelPlay(GLOBAL_BASS_MIXER, 1);
            }
        }
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
            info.current_position as f64
        }

        #[cfg(desktop)]
        unsafe {
            if CURRENT_STREAM == 0 {
                return 0.0;
            }
            let byte_pos = BASS_ChannelGetPosition(CURRENT_STREAM, BASS_POS_BYTE);
            let seconds = BASS_ChannelBytes2Seconds(CURRENT_STREAM, byte_pos);
            seconds * 1000.0
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
                    Some(0.0)
                } else {
                    Some(info.current_position as f64)
                },
                is_playing: if is_reset { true } else { info.is_playing },
            }
        }

        #[cfg(desktop)]
        unsafe {
            let current_position = if is_reset || CURRENT_STREAM == 0 {
                Some(0.0)
            } else {
                let byte_pos = BASS_ChannelGetPosition(CURRENT_STREAM, BASS_POS_BYTE);
                let seconds = BASS_ChannelBytes2Seconds(CURRENT_STREAM, byte_pos);
                Some(seconds * 1000.0)
            };

            let is_playing = if is_reset {
                true
            } else if GLOBAL_BASS_MIXER == 0 {
                false
            } else {
                let state = BASS_ChannelIsActive(GLOBAL_BASS_MIXER);
                state == BASS_ACTIVE_PLAYING
            };

            // Get current index from global state
            let index = GLOBAL_PLAYER_STATE
                .get()
                .and_then(|state| state.lock().ok())
                .and_then(|state| state.current_index)
                .map(|i| i as i64)
                .unwrap_or(-1);

            MusicPlayerSync {
                index,
                current_position,
                is_playing,
            }
        }
    }

    pub fn add_playlist(playlist: Vec<MusicMetadata>) {
        #[cfg(desktop)]
        {
            if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
                if let Ok(mut state) = state_lock.lock() {
                    let was_empty = state.playlist.is_empty();

                    for music in playlist {
                        state.playlist.push(PlaylistItem {
                            metadata: music,
                        });
                    }

                    // Auto-play first track if nothing is playing
                    if was_empty && !state.playlist.is_empty() && state.current_index.is_none() {
                        drop(state); // Release lock before calling goto_playlist
                        Self::goto_playlist(0);
                    }
                }
            }
        }

        #[cfg(target_os = "android")]
        {
            let music_playlist = playlist
                .into_iter()
                .map(|music| PlaylistAddMusic {
                    file_path: music.path,
                    title: music.title.unwrap_or(MusicMetadata::default_title().to_string()),
                    artist: music.artist.unwrap_or(MusicMetadata::default_artist().to_string()),
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
        {
            if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
                if let Ok(mut state) = state_lock.lock() {
                    if index >= state.playlist.len() {
                        return;
                    }

                    // Free stream if it's the current one
                    if let Some(current) = state.current_index {
                        if current == index {
                            drop(state); // Release lock before calling stop
                            Self::stop_current_stream();
                            if let Ok(mut state) = state_lock.lock() {
                                state.current_index = None;
                                state.playlist.remove(index);
                            }
                            return;
                        }
                    }

                    state.playlist.remove(index);

                    // Adjust current index
                    if let Some(current) = state.current_index {
                        if index < current {
                            state.current_index = Some(current - 1);
                        }
                    }
                }
            }
        }
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
        {
            if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
                let music = {
                    let state = state_lock.lock().unwrap();
                    if index >= state.playlist.len() {
                        return;
                    }
                    state.playlist[index].metadata.clone()
                };

                Self::stop_current_stream();

                if Self::load_music(music) {
                    if let Ok(mut state) = state_lock.lock() {
                        state.current_index = Some(index);
                    }
                    Self::play_pause(true);
                    Self::emit_sync(true);
                }
            }
        }
    }

    #[cfg(desktop)]
    fn stop_current_stream() {
        unsafe {
            if CURRENT_STREAM != 0 {
                BASS_ChannelStop(CURRENT_STREAM);
                BASS_Mixer_ChannelRemove(CURRENT_STREAM);
                BASS_StreamFree(CURRENT_STREAM);
                CURRENT_STREAM = 0;
            }
            // Also clear the mixer buffer
            if GLOBAL_BASS_MIXER != 0 {
                BASS_ChannelSetPosition(GLOBAL_BASS_MIXER, 0, BASS_POS_BYTE);
            }
        }
    }

    #[cfg(desktop)]
    fn clear_playlist() {
        unsafe {
            // Completely stop and clear the mixer
            if GLOBAL_BASS_MIXER != 0 {
                BASS_ChannelStop(GLOBAL_BASS_MIXER);
                BASS_ChannelSetPosition(GLOBAL_BASS_MIXER, 0, BASS_POS_BYTE);
            }
        }
        Self::stop_current_stream();
        if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
            if let Ok(mut state) = state_lock.lock() {
                state.playlist.clear();
                state.current_index = None;
            }
        }
    }

    #[cfg(desktop)]
    fn play_next() {
        if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
            let next_index = {
                let state = state_lock.lock().unwrap();
                match (state.current_index, state.repeat_mode) {
                    (Some(current), RepeatMode::One) => Some(current),
                    (Some(current), _) if current + 1 < state.playlist.len() => Some(current + 1),
                    (Some(_), RepeatMode::All) => Some(0),
                    _ => None,
                }
            };

            if let Some(index) = next_index {
                // Get the next music metadata
                let music = {
                    let state = state_lock.lock().unwrap();
                    state.playlist[index].metadata.clone()
                };

                // For gapless playback, remove the old stream and load the new one without stopping the mixer
                unsafe {
                    if CURRENT_STREAM != 0 {
                        BASS_Mixer_ChannelRemove(CURRENT_STREAM);
                        BASS_StreamFree(CURRENT_STREAM);
                        CURRENT_STREAM = 0;
                    }
                }

                if Self::load_music(music) {
                    if let Ok(mut state) = state_lock.lock() {
                        state.current_index = Some(index);
                    }
                    Self::emit_sync(true);
                }
            } else {
                Self::stop_current_stream();
                if let Ok(mut state) = state_lock.lock() {
                    state.current_index = None;
                }
            }
        }
    }

    #[cfg(desktop)]
    fn load_music(music: MusicMetadata) -> bool {
        unsafe {
            let path = CString::new(music.path.clone()).unwrap();
            let stream = BASS_StreamCreateFile(
                false,
                path.as_ptr() as *const _,
                0,
                0,
                BASS_STREAM_DECODE,
            );

            if stream == 0 {
                logger::error!("Failed to load BASS music: {}, error: {}", music.path, BASS_ErrorGetCode());
                return false;
            }

            let ok = BASS_Mixer_StreamAddChannel(GLOBAL_BASS_MIXER, stream, BASS_MIXER_NORAMPIN);
            if ok == 0 {
                logger::error!("Failed to add channel to mixer: {}, error: {}", music.path, BASS_ErrorGetCode());
                BASS_StreamFree(stream);
                return false;
            }

            CURRENT_STREAM = stream;
            logger::info!("Successfully loaded: {}", music.path);
            true
        }
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
        {
            if let Some(state_lock) = GLOBAL_PLAYER_STATE.get() {
                if let Ok(mut state) = state_lock.lock() {
                    if from >= state.playlist.len() || to >= state.playlist.len() {
                        return;
                    }

                    let item = state.playlist.remove(from);
                    state.playlist.insert(to, item);

                    // Adjust current index
                    if let Some(current) = state.current_index {
                        state.current_index = Some(
                            if current == from {
                                to
                            } else if from < current && to >= current {
                                current - 1
                            } else if from > current && to <= current {
                                current + 1
                            } else {
                                current
                            }
                        );
                    }
                }
            }
        }
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
        unsafe {
            if GLOBAL_BASS_MIXER != 0 {
                let clamped_volume = volume.max(0.0).min(1.0);
                if BASS_ChannelSetAttribute(GLOBAL_BASS_MIXER, BASS_ATTRIB_VOL, clamped_volume) == 0 {
                    logger::error!("Failed to set volume, error: {}", BASS_ErrorGetCode());
                }
            }
        }
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
        unsafe {
            if GLOBAL_BASS_MIXER == 0 {
                return;
            }

            if play {
                if BASS_ChannelPlay(GLOBAL_BASS_MIXER, 0) == 0 {
                    logger::error!("Failed to play, error: {}", BASS_ErrorGetCode());
                }
            } else {
                if BASS_ChannelPause(GLOBAL_BASS_MIXER) == 0 {
                    logger::error!("Failed to pause, error: {}", BASS_ErrorGetCode());
                }
            }
        }
    }

    pub fn equalizer(values: Vec<f32>) {
        // Note: BASS equalizer implementation would require BASS_FX plugin
        // and DSP effects setup. This is a placeholder for future implementation.
        #[cfg(desktop)]
        {
            logger::info!("Equalizer called with {} bands (not yet implemented)", values.len());
            // TODO: Implement BASS_FX equalizer with proper DSP chain
        }
    }

    pub fn reset_equalizer() {
        #[cfg(desktop)]
        {
            logger::info!("Reset equalizer (not yet implemented)");
            // TODO: Clear BASS_FX equalizer DSP chain
        }
    }

    pub fn toggle_bit_perfect(enable: bool) {
        #[cfg(desktop)]
        {
            logger::info!("Bit-perfect mode toggle (not yet implemented for BASS): {}", enable);
            // TODO: Configure BASS for bit-perfect playback if supported
            // This may require specific device initialization flags
        }
    }

    pub fn request_sync() {
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
            // Spawn a thread to monitor playback state
            thread::spawn(move || {
                loop {
                    thread::sleep(std::time::Duration::from_millis(100));

                    unsafe {
                        if CURRENT_STREAM != 0 {
                            let state = BASS_ChannelIsActive(CURRENT_STREAM);
                            if state == BASS_ACTIVE_STOPPED {
                                // Track ended, trigger next
                                logger::info!("Track ended, playing next");
                                Self::play_next();
                            }
                        }
                    }
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

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        #[cfg(desktop)]
        unsafe {
            Self::stop_current_stream();
            if GLOBAL_BASS_MIXER != 0 {
                BASS_StreamFree(GLOBAL_BASS_MIXER);
                GLOBAL_BASS_MIXER = 0;
            }
            BASS_Free();
            logger::info!("BASS cleaned up");
        }
    }
}