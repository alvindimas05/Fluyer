use crate::music::metadata::MusicMetadata;
use crate::state::{app_handle, main_window};
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::ptr;
use std::sync::{Mutex, OnceLock};
use std::thread;
use tauri::Emitter;

const BASS_PLUGINS: [&str; 1] = ["bassflac"];

const BASS_SAMPLE_FLOAT: u32 = 0x100;
const BASS_STREAM_DECODE: u32 = 0x200000;
const BASS_MIXER_NORAMPIN: u32 = 0x800000;
const BASS_ACTIVE_STOPPED: u32 = 0;
const BASS_ACTIVE_PLAYING: u32 = 1;
#[allow(dead_code)]
const BASS_ACTIVE_PAUSED: u32 = 3;
const BASS_POS_BYTE: u32 = 0;
const BASS_ATTRIB_VOL: u32 = 2;

#[cfg(desktop)]
#[link(name = "bass")]
#[link(name = "bassmix")]
extern "C" {
    fn BASS_Init(
        device: i32,
        freq: u32,
        flags: u32,
        win: *mut std::ffi::c_void,
        clsid: *mut std::ffi::c_void,
    ) -> i32;
    fn BASS_PluginLoad(file: *const std::ffi::c_char, flags: u32) -> u32;
    #[allow(dead_code)]
    fn BASS_PluginFree(handle: u32) -> i32;
    fn BASS_StreamCreateFile(
        mem: bool,
        file: *const std::ffi::c_void,
        offset: u64,
        length: u64,
        flags: u32,
    ) -> u32;
    fn BASS_Mixer_StreamCreate(freq: u32, chans: u32, flags: u32) -> u32;
    fn BASS_Mixer_StreamAddChannel(handle: u32, channel: u32, flags: u32) -> i32;
    fn BASS_Mixer_ChannelRemove(handle: u32) -> u32;
    #[allow(dead_code)]
    fn BASS_Mixer_ChannelIsActive(handle: u32) -> u32;
    fn BASS_StreamFree(handle: u32) -> i32;
    fn BASS_ChannelPlay(handle: u32, restart: i32) -> i32;
    fn BASS_ChannelPause(handle: u32) -> i32;
    fn BASS_ChannelStop(handle: u32) -> i32;
    fn BASS_ChannelIsActive(handle: u32) -> u32;
    #[allow(dead_code)]
    fn BASS_ChannelGetLength(handle: u32, mode: u32) -> u64;
    fn BASS_ChannelGetPosition(handle: u32, mode: u32) -> u64;
    fn BASS_ChannelSetPosition(handle: u32, pos: u64, mode: u32) -> i32;
    fn BASS_ChannelBytes2Seconds(handle: u32, pos: u64) -> f64;
    fn BASS_ChannelSeconds2Bytes(handle: u32, pos: f64) -> u64;
    fn BASS_ChannelSetAttribute(handle: u32, attrib: u32, value: f32) -> i32;
    #[allow(dead_code)]
    fn BASS_ChannelGetAttribute(handle: u32, attrib: u32, value: *mut f32) -> i32;
    fn BASS_ErrorGetCode() -> i32;
    fn BASS_Free() -> i32;
}

// Android BASS library loaded dynamically
#[cfg(target_os = "android")]
mod bass_android {
    use libloading::{Library, Symbol};
    use std::ffi::c_void;
    use std::sync::OnceLock;

    pub struct BassLibrary {
        _bass: Library,
        _bassmix: Library,
        // BASS functions
        pub bass_init: unsafe extern "C" fn(i32, u32, u32, *mut c_void, *mut c_void) -> i32,
        pub bass_plugin_load: unsafe extern "C" fn(*const i8, u32) -> u32,
        pub bass_stream_create_file:
            unsafe extern "C" fn(bool, *const c_void, u64, u64, u32) -> u32,
        pub bass_stream_free: unsafe extern "C" fn(u32) -> i32,
        pub bass_channel_play: unsafe extern "C" fn(u32, i32) -> i32,
        pub bass_channel_pause: unsafe extern "C" fn(u32) -> i32,
        pub bass_channel_stop: unsafe extern "C" fn(u32) -> i32,
        pub bass_channel_is_active: unsafe extern "C" fn(u32) -> u32,
        pub bass_channel_get_position: unsafe extern "C" fn(u32, u32) -> u64,
        pub bass_channel_set_position: unsafe extern "C" fn(u32, u64, u32) -> i32,
        pub bass_channel_bytes2seconds: unsafe extern "C" fn(u32, u64) -> f64,
        pub bass_channel_seconds2bytes: unsafe extern "C" fn(u32, f64) -> u64,
        pub bass_channel_set_attribute: unsafe extern "C" fn(u32, u32, f32) -> i32,
        pub bass_error_get_code: unsafe extern "C" fn() -> i32,
        pub bass_free: unsafe extern "C" fn() -> i32,
        // BASSMIX functions
        pub bass_mixer_stream_create: unsafe extern "C" fn(u32, u32, u32) -> u32,
        pub bass_mixer_stream_add_channel: unsafe extern "C" fn(u32, u32, u32) -> i32,
        pub bass_mixer_channel_remove: unsafe extern "C" fn(u32) -> u32,
    }

    unsafe impl Send for BassLibrary {}
    unsafe impl Sync for BassLibrary {}

    static BASS_LIB: OnceLock<BassLibrary> = OnceLock::new();

    pub fn get_bass() -> Option<&'static BassLibrary> {
        BASS_LIB.get()
    }

    pub fn initialize_bass() -> Result<(), String> {
        if BASS_LIB.get().is_some() {
            return Ok(());
        }

        unsafe {
            let bass = Library::new("libbass.so")
                .map_err(|e| format!("Failed to load libbass.so: {}", e))?;
            let bassmix = Library::new("libbassmix.so")
                .map_err(|e| format!("Failed to load libbassmix.so: {}", e))?;

            // Load BASS functions - extract raw function pointers before moving libraries
            let bass_init_fn: unsafe extern "C" fn(i32, u32, u32, *mut c_void, *mut c_void) -> i32 =
                *bass
                    .get::<unsafe extern "C" fn(i32, u32, u32, *mut c_void, *mut c_void) -> i32>(
                        b"BASS_Init",
                    )
                    .map_err(|e| format!("Failed to load BASS_Init: {}", e))?;
            let bass_plugin_load_fn: unsafe extern "C" fn(*const i8, u32) -> u32 = *bass
                .get::<unsafe extern "C" fn(*const i8, u32) -> u32>(b"BASS_PluginLoad")
                .map_err(|e| format!("Failed to load BASS_PluginLoad: {}", e))?;
            let bass_stream_create_file_fn: unsafe extern "C" fn(
                bool,
                *const c_void,
                u64,
                u64,
                u32,
            ) -> u32 = *bass
                .get::<unsafe extern "C" fn(bool, *const c_void, u64, u64, u32) -> u32>(
                    b"BASS_StreamCreateFile",
                )
                .map_err(|e| format!("Failed to load BASS_StreamCreateFile: {}", e))?;
            let bass_stream_free_fn: unsafe extern "C" fn(u32) -> i32 = *bass
                .get::<unsafe extern "C" fn(u32) -> i32>(b"BASS_StreamFree")
                .map_err(|e| format!("Failed to load BASS_StreamFree: {}", e))?;
            let bass_channel_play_fn: unsafe extern "C" fn(u32, i32) -> i32 = *bass
                .get::<unsafe extern "C" fn(u32, i32) -> i32>(b"BASS_ChannelPlay")
                .map_err(|e| format!("Failed to load BASS_ChannelPlay: {}", e))?;
            let bass_channel_pause_fn: unsafe extern "C" fn(u32) -> i32 = *bass
                .get::<unsafe extern "C" fn(u32) -> i32>(b"BASS_ChannelPause")
                .map_err(|e| format!("Failed to load BASS_ChannelPause: {}", e))?;
            let bass_channel_stop_fn: unsafe extern "C" fn(u32) -> i32 = *bass
                .get::<unsafe extern "C" fn(u32) -> i32>(b"BASS_ChannelStop")
                .map_err(|e| format!("Failed to load BASS_ChannelStop: {}", e))?;
            let bass_channel_is_active_fn: unsafe extern "C" fn(u32) -> u32 = *bass
                .get::<unsafe extern "C" fn(u32) -> u32>(b"BASS_ChannelIsActive")
                .map_err(|e| format!("Failed to load BASS_ChannelIsActive: {}", e))?;
            let bass_channel_get_position_fn: unsafe extern "C" fn(u32, u32) -> u64 = *bass
                .get::<unsafe extern "C" fn(u32, u32) -> u64>(b"BASS_ChannelGetPosition")
                .map_err(|e| format!("Failed to load BASS_ChannelGetPosition: {}", e))?;
            let bass_channel_set_position_fn: unsafe extern "C" fn(u32, u64, u32) -> i32 = *bass
                .get::<unsafe extern "C" fn(u32, u64, u32) -> i32>(b"BASS_ChannelSetPosition")
                .map_err(|e| format!("Failed to load BASS_ChannelSetPosition: {}", e))?;
            let bass_channel_bytes2seconds_fn: unsafe extern "C" fn(u32, u64) -> f64 = *bass
                .get::<unsafe extern "C" fn(u32, u64) -> f64>(b"BASS_ChannelBytes2Seconds")
                .map_err(|e| format!("Failed to load BASS_ChannelBytes2Seconds: {}", e))?;
            let bass_channel_seconds2bytes_fn: unsafe extern "C" fn(u32, f64) -> u64 = *bass
                .get::<unsafe extern "C" fn(u32, f64) -> u64>(b"BASS_ChannelSeconds2Bytes")
                .map_err(|e| format!("Failed to load BASS_ChannelSeconds2Bytes: {}", e))?;
            let bass_channel_set_attribute_fn: unsafe extern "C" fn(u32, u32, f32) -> i32 = *bass
                .get::<unsafe extern "C" fn(u32, u32, f32) -> i32>(b"BASS_ChannelSetAttribute")
                .map_err(|e| format!("Failed to load BASS_ChannelSetAttribute: {}", e))?;
            let bass_error_get_code_fn: unsafe extern "C" fn() -> i32 = *bass
                .get::<unsafe extern "C" fn() -> i32>(b"BASS_ErrorGetCode")
                .map_err(|e| format!("Failed to load BASS_ErrorGetCode: {}", e))?;
            let bass_free_fn: unsafe extern "C" fn() -> i32 = *bass
                .get::<unsafe extern "C" fn() -> i32>(b"BASS_Free")
                .map_err(|e| format!("Failed to load BASS_Free: {}", e))?;

            // Load BASSMIX functions
            let bass_mixer_stream_create_fn: unsafe extern "C" fn(u32, u32, u32) -> u32 = *bassmix
                .get::<unsafe extern "C" fn(u32, u32, u32) -> u32>(b"BASS_Mixer_StreamCreate")
                .map_err(|e| format!("Failed to load BASS_Mixer_StreamCreate: {}", e))?;
            let bass_mixer_stream_add_channel_fn: unsafe extern "C" fn(u32, u32, u32) -> i32 =
                *bassmix
                    .get::<unsafe extern "C" fn(u32, u32, u32) -> i32>(
                        b"BASS_Mixer_StreamAddChannel",
                    )
                    .map_err(|e| format!("Failed to load BASS_Mixer_StreamAddChannel: {}", e))?;
            let bass_mixer_channel_remove_fn: unsafe extern "C" fn(u32) -> u32 = *bassmix
                .get::<unsafe extern "C" fn(u32) -> u32>(b"BASS_Mixer_ChannelRemove")
                .map_err(|e| format!("Failed to load BASS_Mixer_ChannelRemove: {}", e))?;

            let lib = BassLibrary {
                _bass: bass,
                _bassmix: bassmix,
                bass_init: bass_init_fn,
                bass_plugin_load: bass_plugin_load_fn,
                bass_stream_create_file: bass_stream_create_file_fn,
                bass_stream_free: bass_stream_free_fn,
                bass_channel_play: bass_channel_play_fn,
                bass_channel_pause: bass_channel_pause_fn,
                bass_channel_stop: bass_channel_stop_fn,
                bass_channel_is_active: bass_channel_is_active_fn,
                bass_channel_get_position: bass_channel_get_position_fn,
                bass_channel_set_position: bass_channel_set_position_fn,
                bass_channel_bytes2seconds: bass_channel_bytes2seconds_fn,
                bass_channel_seconds2bytes: bass_channel_seconds2bytes_fn,
                bass_channel_set_attribute: bass_channel_set_attribute_fn,
                bass_error_get_code: bass_error_get_code_fn,
                bass_free: bass_free_fn,
                bass_mixer_stream_create: bass_mixer_stream_create_fn,
                bass_mixer_stream_add_channel: bass_mixer_stream_add_channel_fn,
                bass_mixer_channel_remove: bass_mixer_channel_remove_fn,
            };

            BASS_LIB
                .set(lib)
                .map_err(|_| "Failed to set BASS library")?;
            Ok(())
        }
    }
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

static mut BASS_MIXER: u32 = 0;
static mut CURRENT_STREAM: u32 = 0;
static PLAYER_STATE: OnceLock<Mutex<PlayerState>> = OnceLock::new();

#[derive(Debug, Clone)]
struct PlayerState {
    playlist: Vec<PlaylistItem>,
    current_index: Option<usize>,
    repeat_mode: RepeatMode,
}

impl MusicPlayer {
    pub fn spawn() -> Self {
        Self::start_focus_listener();

        // Initialize global player state
        PLAYER_STATE.get_or_init(|| {
            Mutex::new(PlayerState {
                playlist: Vec::new(),
                current_index: None,
                repeat_mode: RepeatMode::None,
            })
        });

        #[cfg(desktop)]
        {
            unsafe {
                if BASS_Init(-1, 44100, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
                    crate::error!("Failed to initialize BASS, error: {}", BASS_ErrorGetCode());
                } else {
                    crate::info!("BASS initialized successfully");
                }

                // Load plugins based on platform
                #[cfg(target_os = "macos")]
                let extension = "dylib";
                #[cfg(target_os = "windows")]
                let extension = "dll";
                #[cfg(target_os = "linux")]
                let extension = "so";

                for plugin in BASS_PLUGINS {
                    #[cfg(not(target_os = "linux"))]
                    let c_path = CString::new(format!("{}.{}", plugin, extension)).unwrap();
                    #[cfg(target_os = "linux")]
                    let c_path = CString::new(format!("lib{}.{}", plugin, extension)).unwrap();

                    let handle = BASS_PluginLoad(c_path.as_ptr(), 0);

                    if handle == 0 {
                        crate::warn!(
                            "Failed to load plugin: {}, error: {}",
                            plugin,
                            BASS_ErrorGetCode()
                        );
                    } else {
                        crate::info!("Loaded plugin: {}", plugin);
                    }
                }

                BASS_MIXER = BASS_Mixer_StreamCreate(44100, 2, BASS_SAMPLE_FLOAT);
                if BASS_MIXER == 0 {
                    crate::error!(
                        "Failed to create BASS mixer stream, error: {}",
                        BASS_ErrorGetCode()
                    );
                } else {
                    crate::info!("BASS mixer created successfully");
                }
            }
        }

        #[cfg(target_os = "android")]
        {
            if let Err(e) = bass_android::initialize_bass() {
                crate::error!("Failed to initialize BASS on Android: {}", e);
            } else {
                crate::info!("BASS libraries loaded successfully on Android");

                if let Some(bass) = bass_android::get_bass() {
                    unsafe {
                        if (bass.bass_init)(-1, 44100, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
                            crate::error!(
                                "Failed to initialize BASS, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        } else {
                            crate::info!("BASS initialized successfully");
                        }

                        // Load FLAC plugin
                        let c_path = CString::new("libbassflac.so").unwrap();
                        let handle = (bass.bass_plugin_load)(c_path.as_ptr() as *const i8, 0);
                        if handle == 0 {
                            crate::warn!(
                                "Failed to load bassflac plugin, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        } else {
                            crate::info!("Loaded bassflac plugin");
                        }

                        BASS_MIXER = (bass.bass_mixer_stream_create)(44100, 2, BASS_SAMPLE_FLOAT);
                        if BASS_MIXER == 0 {
                            crate::error!(
                                "Failed to create BASS mixer stream, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        } else {
                            crate::info!("BASS mixer created successfully");
                        }
                    }
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
            Self::clear_playlist();
            return;
        }

        if _command == MusicCommand::Next {
            Self::play_next();
            return;
        }

        if _command == MusicCommand::Repeat
            || _command == MusicCommand::RepeatOne
            || _command == MusicCommand::RepeatNone
        {
            if let Some(state_lock) = PLAYER_STATE.get() {
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
    }

    pub fn set_pos(position: u64) {
        #[cfg(desktop)]
        unsafe {
            if CURRENT_STREAM != 0 && BASS_MIXER != 0 {
                // Pause the mixer to prevent delay from buffered data
                BASS_ChannelPause(BASS_MIXER);

                // Set the new position on the source stream
                let seconds = position as f64 / 1000.0;
                let byte_pos = BASS_ChannelSeconds2Bytes(CURRENT_STREAM, seconds);
                if BASS_ChannelSetPosition(CURRENT_STREAM, byte_pos, BASS_POS_BYTE) == 0 {
                    crate::error!("Failed to set position, error: {}", BASS_ErrorGetCode());
                }

                // Restart the mixer with restart=true to clear the buffer
                BASS_ChannelPlay(BASS_MIXER, 1);
            }
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    if CURRENT_STREAM != 0 && BASS_MIXER != 0 {
                        (bass.bass_channel_pause)(BASS_MIXER);
                        let seconds = position as f64 / 1000.0;
                        let byte_pos = (bass.bass_channel_seconds2bytes)(CURRENT_STREAM, seconds);
                        if (bass.bass_channel_set_position)(CURRENT_STREAM, byte_pos, BASS_POS_BYTE)
                            == 0
                        {
                            crate::error!(
                                "Failed to set position, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        }
                        (bass.bass_channel_play)(BASS_MIXER, 1);
                    }
                }
            }
        }
    }

    pub fn get_current_duration() -> f64 {
        #[cfg(desktop)]
        unsafe {
            if CURRENT_STREAM == 0 {
                return 0.0;
            }
            let byte_pos = BASS_ChannelGetPosition(CURRENT_STREAM, BASS_POS_BYTE);
            let seconds = BASS_ChannelBytes2Seconds(CURRENT_STREAM, byte_pos);
            return seconds * 1000.0;
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    if CURRENT_STREAM == 0 {
                        return 0.0;
                    }
                    let byte_pos = (bass.bass_channel_get_position)(CURRENT_STREAM, BASS_POS_BYTE);
                    let seconds = (bass.bass_channel_bytes2seconds)(CURRENT_STREAM, byte_pos);
                    return seconds * 1000.0;
                }
            }
            0.0
        }
    }

    pub fn get_sync_info(is_reset: bool) -> MusicPlayerSync {
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
            } else if BASS_MIXER == 0 {
                false
            } else {
                let state = BASS_ChannelIsActive(BASS_MIXER);
                state == BASS_ACTIVE_PLAYING
            };

            // Get current index from global state
            let index = PLAYER_STATE
                .get()
                .and_then(|state| state.lock().ok())
                .and_then(|state| state.current_index)
                .map(|i| i as i64)
                .unwrap_or(-1);

            return MusicPlayerSync {
                index,
                current_position,
                is_playing,
            };
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    let current_position = if is_reset || CURRENT_STREAM == 0 {
                        Some(0.0)
                    } else {
                        let byte_pos =
                            (bass.bass_channel_get_position)(CURRENT_STREAM, BASS_POS_BYTE);
                        let seconds = (bass.bass_channel_bytes2seconds)(CURRENT_STREAM, byte_pos);
                        Some(seconds * 1000.0)
                    };

                    let is_playing = if is_reset {
                        true
                    } else if BASS_MIXER == 0 {
                        false
                    } else {
                        let state = (bass.bass_channel_is_active)(BASS_MIXER);
                        state == BASS_ACTIVE_PLAYING
                    };

                    let index = PLAYER_STATE
                        .get()
                        .and_then(|state| state.lock().ok())
                        .and_then(|state| state.current_index)
                        .map(|i| i as i64)
                        .unwrap_or(-1);

                    return MusicPlayerSync {
                        index,
                        current_position,
                        is_playing,
                    };
                }
            }

            MusicPlayerSync {
                index: -1,
                current_position: Some(0.0),
                is_playing: false,
            }
        }
    }

    pub fn add_playlist(playlist: Vec<MusicMetadata>) {
        if let Some(state_lock) = PLAYER_STATE.get() {
            if let Ok(mut state) = state_lock.lock() {
                let was_empty = state.playlist.is_empty();

                for music in playlist {
                    state.playlist.push(PlaylistItem { metadata: music });
                }

                // Auto-play first track if nothing is playing
                if was_empty && !state.playlist.is_empty() && state.current_index.is_none() {
                    drop(state); // Release lock before calling goto_playlist
                    Self::goto_playlist(0);
                }
            }
        }
    }

    pub fn remove_playlist(index: usize) {
        if let Some(state_lock) = PLAYER_STATE.get() {
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

    pub fn goto_playlist(index: usize) {
        if let Some(state_lock) = PLAYER_STATE.get() {
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

    fn stop_current_stream() {
        #[cfg(desktop)]
        unsafe {
            if CURRENT_STREAM != 0 {
                BASS_ChannelStop(CURRENT_STREAM);
                BASS_Mixer_ChannelRemove(CURRENT_STREAM);
                BASS_StreamFree(CURRENT_STREAM);
                CURRENT_STREAM = 0;
            }
            // Also clear the mixer buffer
            if BASS_MIXER != 0 {
                BASS_ChannelSetPosition(BASS_MIXER, 0, BASS_POS_BYTE);
            }
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    if CURRENT_STREAM != 0 {
                        (bass.bass_channel_stop)(CURRENT_STREAM);
                        (bass.bass_mixer_channel_remove)(CURRENT_STREAM);
                        (bass.bass_stream_free)(CURRENT_STREAM);
                        CURRENT_STREAM = 0;
                    }
                    if BASS_MIXER != 0 {
                        (bass.bass_channel_set_position)(BASS_MIXER, 0, BASS_POS_BYTE);
                    }
                }
            }
        }
    }

    fn clear_playlist() {
        #[cfg(desktop)]
        unsafe {
            // Completely stop and clear the mixer
            if BASS_MIXER != 0 {
                BASS_ChannelStop(BASS_MIXER);
                BASS_ChannelSetPosition(BASS_MIXER, 0, BASS_POS_BYTE);
            }
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    if BASS_MIXER != 0 {
                        (bass.bass_channel_stop)(BASS_MIXER);
                        (bass.bass_channel_set_position)(BASS_MIXER, 0, BASS_POS_BYTE);
                    }
                }
            }
        }

        Self::stop_current_stream();
        if let Some(state_lock) = PLAYER_STATE.get() {
            if let Ok(mut state) = state_lock.lock() {
                state.playlist.clear();
                state.current_index = None;
            }
        }
    }

    fn play_next() {
        if let Some(state_lock) = PLAYER_STATE.get() {
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
                #[cfg(desktop)]
                unsafe {
                    if CURRENT_STREAM != 0 {
                        BASS_Mixer_ChannelRemove(CURRENT_STREAM);
                        BASS_StreamFree(CURRENT_STREAM);
                        CURRENT_STREAM = 0;
                    }
                }

                #[cfg(target_os = "android")]
                {
                    if let Some(bass) = bass_android::get_bass() {
                        unsafe {
                            if CURRENT_STREAM != 0 {
                                (bass.bass_mixer_channel_remove)(CURRENT_STREAM);
                                (bass.bass_stream_free)(CURRENT_STREAM);
                                CURRENT_STREAM = 0;
                            }
                        }
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

    fn load_music(music: MusicMetadata) -> bool {
        #[cfg(desktop)]
        unsafe {
            let path = CString::new(music.path.clone()).unwrap();
            let stream =
                BASS_StreamCreateFile(false, path.as_ptr() as *const _, 0, 0, BASS_STREAM_DECODE);

            if stream == 0 {
                crate::error!(
                    "Failed to load BASS music: {}, error: {}",
                    music.path,
                    BASS_ErrorGetCode()
                );
                return false;
            }

            let ok = BASS_Mixer_StreamAddChannel(BASS_MIXER, stream, BASS_MIXER_NORAMPIN);
            if ok == 0 {
                crate::error!(
                    "Failed to add channel to mixer: {}, error: {}",
                    music.path,
                    BASS_ErrorGetCode()
                );
                BASS_StreamFree(stream);
                return false;
            }

            CURRENT_STREAM = stream;
            crate::info!("Successfully loaded: {}", music.path);
            return true;
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    let path = CString::new(music.path.clone()).unwrap();
                    let stream = (bass.bass_stream_create_file)(
                        false,
                        path.as_ptr() as *const _,
                        0,
                        0,
                        BASS_STREAM_DECODE,
                    );

                    if stream == 0 {
                        crate::error!(
                            "Failed to load BASS music: {}, error: {}",
                            music.path,
                            (bass.bass_error_get_code)()
                        );
                        return false;
                    }

                    let ok = (bass.bass_mixer_stream_add_channel)(
                        BASS_MIXER,
                        stream,
                        BASS_MIXER_NORAMPIN,
                    );
                    if ok == 0 {
                        crate::error!(
                            "Failed to add channel to mixer: {}, error: {}",
                            music.path,
                            (bass.bass_error_get_code)()
                        );
                        (bass.bass_stream_free)(stream);
                        return false;
                    }

                    CURRENT_STREAM = stream;
                    crate::info!("Successfully loaded: {}", music.path);
                    return true;
                }
            }
            false
        }
    }

    pub fn moveto_playlist(from: usize, to: usize) {
        if let Some(state_lock) = PLAYER_STATE.get() {
            if let Ok(mut state) = state_lock.lock() {
                if from >= state.playlist.len() || to >= state.playlist.len() {
                    return;
                }

                let item = state.playlist.remove(from);
                state.playlist.insert(to, item);

                // Adjust current index
                if let Some(current) = state.current_index {
                    state.current_index = Some(if current == from {
                        to
                    } else if from < current && to >= current {
                        current - 1
                    } else if from > current && to <= current {
                        current + 1
                    } else {
                        current
                    });
                }
            }
        }
    }

    pub fn set_volume(volume: f32) {
        #[cfg(desktop)]
        unsafe {
            if BASS_MIXER != 0 {
                let clamped_volume = volume.max(0.0).min(1.0);
                if BASS_ChannelSetAttribute(BASS_MIXER, BASS_ATTRIB_VOL, clamped_volume) == 0 {
                    crate::error!("Failed to set volume, error: {}", BASS_ErrorGetCode());
                }
            }
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    if BASS_MIXER != 0 {
                        let clamped_volume = volume.max(0.0).min(1.0);
                        if (bass.bass_channel_set_attribute)(
                            BASS_MIXER,
                            BASS_ATTRIB_VOL,
                            clamped_volume,
                        ) == 0
                        {
                            crate::error!(
                                "Failed to set volume, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        }
                    }
                }
            }
        }
    }

    fn play_pause(play: bool) {
        #[cfg(desktop)]
        unsafe {
            if BASS_MIXER == 0 {
                return;
            }

            if play {
                if BASS_ChannelPlay(BASS_MIXER, 0) == 0 {
                    crate::error!("Failed to play, error: {}", BASS_ErrorGetCode());
                }
            } else {
                if BASS_ChannelPause(BASS_MIXER) == 0 {
                    crate::error!("Failed to pause, error: {}", BASS_ErrorGetCode());
                }
            }
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    if BASS_MIXER == 0 {
                        return;
                    }

                    if play {
                        if (bass.bass_channel_play)(BASS_MIXER, 0) == 0 {
                            crate::error!(
                                "Failed to play, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        }
                    } else {
                        if (bass.bass_channel_pause)(BASS_MIXER) == 0 {
                            crate::error!(
                                "Failed to pause, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn equalizer(values: Vec<f32>) {
        // Note: BASS equalizer implementation would require BASS_FX plugin
        // and DSP effects setup. This is a placeholder for future implementation.
        crate::info!(
            "Equalizer called with {} bands (not yet implemented)",
            values.len()
        );
        // TODO: Implement BASS_FX equalizer with proper DSP chain
    }

    pub fn reset_equalizer() {
        crate::info!("Reset equalizer (not yet implemented)");
        // TODO: Clear BASS_FX equalizer DSP chain
    }

    pub fn toggle_bit_perfect(enable: bool) {
        crate::info!(
            "Bit-perfect mode toggle (not yet implemented for BASS): {}",
            enable
        );
        // TODO: Configure BASS for bit-perfect playback if supported
        // This may require specific device initialization flags
    }

    pub fn request_sync() {
        Self::emit_sync(false);
    }

    pub fn emit_sync(is_reset: bool) {
        app_handle()
            .emit(
                crate::commands::route::MUSIC_PLAYER_SYNC,
                Self::get_sync_info(is_reset),
            )
            .unwrap();
    }

    pub fn start_listener() {
        // Spawn a thread to monitor playback state
        thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(100));

                #[cfg(desktop)]
                unsafe {
                    if CURRENT_STREAM != 0 {
                        let state = BASS_ChannelIsActive(CURRENT_STREAM);
                        if state == BASS_ACTIVE_STOPPED {
                            // Track ended, trigger next
                            crate::info!("Track ended, playing next");
                            Self::play_next();
                        }
                    }
                }

                #[cfg(target_os = "android")]
                {
                    if let Some(bass) = bass_android::get_bass() {
                        unsafe {
                            if CURRENT_STREAM != 0 {
                                let state = (bass.bass_channel_is_active)(CURRENT_STREAM);
                                if state == BASS_ACTIVE_STOPPED {
                                    crate::info!("Track ended, playing next");
                                    Self::play_next();
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    fn start_focus_listener() {
        use tauri::Listener;

        main_window().listen("tauri://focus", move |_| {
            MusicPlayer::emit_sync(false);
        });
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        #[cfg(desktop)]
        unsafe {
            Self::stop_current_stream();
            if BASS_MIXER != 0 {
                BASS_StreamFree(BASS_MIXER);
                BASS_MIXER = 0;
            }
            BASS_Free();
            crate::info!("BASS cleaned up");
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    Self::stop_current_stream();
                    if BASS_MIXER != 0 {
                        (bass.bass_stream_free)(BASS_MIXER);
                        BASS_MIXER = 0;
                    }
                    (bass.bass_free)();
                    crate::info!("BASS cleaned up");
                }
            }
        }
    }
}
