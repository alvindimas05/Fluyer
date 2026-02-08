use crate::music::metadata::MusicMetadata;
use crate::state::{app_handle, main_window};
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use tauri::{Emitter, Manager};

#[cfg(target_os = "android")]
use tauri_plugin_fluyer::FluyerExt;

const BASS_PLUGINS: [&str; 6] = [
    "bassflac", "bassopus", "bassape", "bassalac", "basswv", "bass_aac",
];

const BASS_SAMPLE_FLOAT: u32 = 0x100;
const BASS_STREAM_DECODE: u32 = 0x200000;
const BASS_MIXER_NORAMPIN: u32 = 0x800000;
const BASS_ACTIVE_STOPPED: u32 = 0;
const BASS_ACTIVE_PLAYING: u32 = 1;
#[allow(dead_code)]
const BASS_ACTIVE_PAUSED: u32 = 3;
const BASS_POS_BYTE: u32 = 0;
const BASS_ATTRIB_VOL: u32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BASS_DEVICEINFO {
    pub name: *const std::ffi::c_char,
    pub driver: *const std::ffi::c_char,
    pub flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BASS_INFO {
    pub flags: u32,
    pub hwsize: u32,
    pub hwfree: u32,
    pub freesam: u32,
    pub free3d: u32,
    pub minrate: u32,
    pub maxrate: u32,
    pub eax: u32,
    pub minbuf: u32,
    pub dsver: u32,
    pub latency: u32,
    pub initflags: u32,
    pub speakers: u32,
    pub freq: u32,
}

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
    fn BASS_GetDeviceInfo(device: u32, info: *mut BASS_DEVICEINFO) -> u32;
    fn BASS_GetInfo(info: *mut BASS_INFO) -> u32;
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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RepeatMode {
    #[serde(rename = "repeatNone")]
    None,
    #[serde(rename = "repeat")]
    All,
    #[serde(rename = "repeatOne")]
    One,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayerSync {
    index: i64,
    current_position: Option<f64>,
    is_playing: bool,
    repeat_mode: RepeatMode,
}

static mut BASS_MIXER: u32 = 0;
static CURRENT_STREAM: AtomicU32 = AtomicU32::new(0);
static PLAYER_STATE: OnceLock<Mutex<PlayerState>> = OnceLock::new();
// Track temporary WAV file created by FFmpeg conversion for cleanup
static TEMP_WAV_PATH: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

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

        // Initialize temp WAV path tracker
        TEMP_WAV_PATH.get_or_init(|| Mutex::new(None));

        #[cfg(desktop)]
        {
            unsafe {
                // List available devices
                let mut i = 0;
                let mut info = std::mem::zeroed::<BASS_DEVICEINFO>();
                while BASS_GetDeviceInfo(i, &mut info) != 0 {
                    let name = if info.name.is_null() {
                        "Unknown".to_string()
                    } else {
                        std::ffi::CStr::from_ptr(info.name)
                            .to_string_lossy()
                            .into_owned()
                    };
                    let driver = if info.driver.is_null() {
                        "Unknown".to_string()
                    } else {
                        std::ffi::CStr::from_ptr(info.driver)
                            .to_string_lossy()
                            .into_owned()
                    };

                    if (info.flags & 2) != 0 {
                        // BASS_DEVICE_DEFAULT
                        crate::info!("Default Audio Device: {} ({})", name, driver);
                    } else if (info.flags & 1) != 0 {
                        // BASS_DEVICE_ENABLED
                        crate::debug!("Available Audio Device {}: {} ({})", i, name, driver);
                    }
                    i += 1;
                }

                // Initialize with 0 for freq to use the device's output rate
                if BASS_Init(-1, 192000, 0, ptr::null_mut(), ptr::null_mut()) == 0 {
                    crate::error!("Failed to initialize BASS, error: {}", BASS_ErrorGetCode());
                } else {
                    let mut info = std::mem::zeroed::<BASS_INFO>();
                    if BASS_GetInfo(&mut info) != 0 {
                        crate::info!(
                            "BASS initialized successfully at {} Hz, Latency: {}ms, MinBuf: {}msle",
                            info.freq,
                            info.latency,
                            info.minbuf
                        );
                    } else {
                        crate::info!("BASS initialized successfully");
                    }
                }

                // Load plugins based on platform
                #[cfg(target_os = "macos")]
                let extension = "dylib";
                #[cfg(target_os = "windows")]
                let extension = "dll";
                #[cfg(target_os = "linux")]
                let extension = "so";

                for plugin in BASS_PLUGINS {
                    #[cfg(target_os = "macos")]
                    if plugin == "bassalac" || plugin == "bass_aac" {
                        continue;
                    }

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

                        // Load plugins
                        for plugin in BASS_PLUGINS {
                            let lib_name = format!("lib{}.so", plugin);
                            let c_path = CString::new(lib_name).unwrap();
                            let handle = (bass.bass_plugin_load)(c_path.as_ptr() as *const i8, 0);
                            if handle == 0 {
                                crate::warn!(
                                    "Failed to load {} plugin, error: {}",
                                    plugin,
                                    (bass.bass_error_get_code)()
                                );
                            } else {
                                crate::info!("Loaded {} plugin", plugin);
                            }
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

        #[cfg(target_os = "android")]
        {
            crate::info!("Initializing Android Media Control");
            let _ = app_handle().fluyer().init_media_control(|event| {
                crate::info!("Media Control Action: {}", event.action);
                if event.action.starts_with("seek:") {
                    if let Ok(pos) = event.action[5..].parse::<u64>() {
                        std::thread::spawn(move || {
                            Self::set_pos(pos);
                        });
                    }
                } else if event.action == "previous" {
                    std::thread::spawn(move || {
                        Self::play_previous();
                    });
                } else {
                    std::thread::spawn(move || {
                        Self::send_command(event.action);
                    });
                }
            });
        }

        Self {}
    }

    pub fn send_command(command: String) {
        let _command = match command.as_str() {
            "play" => MusicCommand::Play,
            "pause" | "stop" => MusicCommand::Pause,
            "next" => MusicCommand::Next,
            "previous" => MusicCommand::None, // Handled separately usually, but mapped to None here if passed directly
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
            Self::play_next(true);
            return;
        }

        // Previous is handled via direct call usually, but if it came through here (though "previous" maps to None above):
        if command == "previous" {
            Self::play_previous();
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
            let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
            if current_stream != 0 && BASS_MIXER != 0 {
                // Pause the mixer to prevent delay from buffered data
                BASS_ChannelPause(BASS_MIXER);

                // Set the new position on the source stream
                let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                let seconds = position as f64 / 1000.0;
                let byte_pos = BASS_ChannelSeconds2Bytes(current_stream, seconds);
                if BASS_ChannelSetPosition(current_stream, byte_pos, BASS_POS_BYTE) == 0 {
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
                    let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                    if current_stream != 0 && BASS_MIXER != 0 {
                        (bass.bass_channel_pause)(BASS_MIXER);
                        let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                        let seconds = position as f64 / 1000.0;
                        let byte_pos = (bass.bass_channel_seconds2bytes)(current_stream, seconds);
                        if (bass.bass_channel_set_position)(current_stream, byte_pos, BASS_POS_BYTE)
                            == 0
                        {
                            crate::error!(
                                "Failed to set position, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        }
                        (bass.bass_channel_play)(BASS_MIXER, 1);
                    }

                    let sync_info = Self::get_sync_info(false);
                    crate::debug!(
                        "Updating media control state after seek: is_playing={}, position={}ms",
                        sync_info.is_playing,
                        position
                    );
                    if let Err(e) = app_handle()
                        .fluyer()
                        .set_media_control_state(sync_info.is_playing, position)
                    {
                        crate::error!("Failed to update media control state: {:?}", e);
                    }
                }
            }
        }
    }

    pub fn get_current_duration() -> f64 {
        #[cfg(desktop)]
        unsafe {
            let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
            if current_stream == 0 {
                return 0.0;
            }
            let byte_pos = BASS_ChannelGetPosition(current_stream, BASS_POS_BYTE);
            let seconds = BASS_ChannelBytes2Seconds(current_stream, byte_pos);
            return seconds * 1000.0;
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                    if current_stream == 0 {
                        return 0.0;
                    }
                    let byte_pos = (bass.bass_channel_get_position)(current_stream, BASS_POS_BYTE);
                    let seconds = (bass.bass_channel_bytes2seconds)(current_stream, byte_pos);
                    return seconds * 1000.0;
                }
            }
            0.0
        }
    }

    pub fn get_sync_info(is_reset: bool) -> MusicPlayerSync {
        #[cfg(desktop)]
        unsafe {
            let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
            let current_position = if is_reset || current_stream == 0 {
                Some(0.0)
            } else {
                let byte_pos = BASS_ChannelGetPosition(current_stream, BASS_POS_BYTE);
                let seconds = BASS_ChannelBytes2Seconds(current_stream, byte_pos);
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

            // Get repeat mode from global state
            let repeat_mode = PLAYER_STATE
                .get()
                .and_then(|state| state.lock().ok())
                .map(|state| state.repeat_mode)
                .unwrap_or(RepeatMode::None);

            return MusicPlayerSync {
                index,
                current_position,
                is_playing,
                repeat_mode,
            };
        }

        #[cfg(target_os = "android")]
        {
            if let Some(bass) = bass_android::get_bass() {
                unsafe {
                    let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                    let current_position = if is_reset || current_stream == 0 {
                        Some(0.0)
                    } else {
                        let byte_pos =
                            (bass.bass_channel_get_position)(current_stream, BASS_POS_BYTE);
                        let seconds = (bass.bass_channel_bytes2seconds)(current_stream, byte_pos);
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

                    let repeat_mode = PLAYER_STATE
                        .get()
                        .and_then(|state| state.lock().ok())
                        .map(|state| state.repeat_mode)
                        .unwrap_or(RepeatMode::None);

                    return MusicPlayerSync {
                        index,
                        current_position,
                        is_playing,
                        repeat_mode,
                    };
                }
            }

            MusicPlayerSync {
                index: -1,
                current_position: Some(0.0),
                is_playing: false,
                repeat_mode: RepeatMode::None,
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
                } else {
                    // Update Android media control boundaries since queue changed
                    #[cfg(target_os = "android")]
                    {
                        let current_index = state.current_index;
                        let total_count = state.playlist.len();
                        drop(state);
                        Self::update_android_media_boundaries(current_index, total_count);
                    }
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

                // Update Android media control boundaries since queue changed
                #[cfg(target_os = "android")]
                {
                    let current_index = state.current_index;
                    let total_count = state.playlist.len();
                    drop(state);
                    Self::update_android_media_boundaries(current_index, total_count);
                }
            }
        }
    }

    pub fn goto_playlist(index: usize) {
        tauri::async_runtime::spawn_blocking(move || {
            if let Some(state_lock) = PLAYER_STATE.get() {
                let (music, total_count) = {
                    let state = match state_lock.lock() {
                        Ok(s) => s,
                        Err(e) => {
                            crate::error!("Failed to lock player state: {}", e);
                            return;
                        }
                    };
                    if index >= state.playlist.len() {
                        return;
                    }
                    (state.playlist[index].metadata.clone(), state.playlist.len())
                };

                Self::stop_current_stream();

                if Self::load_music(music, index, total_count) {
                    if let Ok(mut state) = state_lock.lock() {
                        state.current_index = Some(index);
                    }
                    Self::play_pause(true);
                    Self::emit_sync(true);
                }
            }
        });
    }

    fn stop_current_stream() {
        // Clean up temp WAV file if it exists
        Self::cleanup_temp_wav();

        #[cfg(desktop)]
        unsafe {
            let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
            if current_stream != 0 {
                BASS_ChannelStop(current_stream);
                BASS_Mixer_ChannelRemove(current_stream);
                BASS_StreamFree(current_stream);
                CURRENT_STREAM.store(0, Ordering::SeqCst);
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
                    let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                    if current_stream != 0 {
                        (bass.bass_channel_stop)(current_stream);
                        (bass.bass_mixer_channel_remove)(current_stream);
                        (bass.bass_stream_free)(current_stream);
                        CURRENT_STREAM.store(0, Ordering::SeqCst);
                    }
                    if BASS_MIXER != 0 {
                        (bass.bass_channel_set_position)(BASS_MIXER, 0, BASS_POS_BYTE);
                    }
                }
            }
        }
    }

    /// Clean up temporary WAV file created by FFmpeg conversion
    fn cleanup_temp_wav() {
        if let Some(temp_path_lock) = TEMP_WAV_PATH.get() {
            if let Ok(mut temp_path) = temp_path_lock.lock() {
                if let Some(path) = temp_path.take() {
                    if path.exists() {
                        if let Err(e) = std::fs::remove_file(&path) {
                            crate::warn!("Failed to remove temp WAV file: {}", e);
                        } else {
                            crate::info!("Cleaned up temp WAV file: {}", path.display());
                        }
                    }
                }
            }
        }
    }

    /// Update Android media control with current boundary state (is_first, is_last)
    /// Called when playlist changes to keep notification buttons in sync
    #[cfg(target_os = "android")]
    fn update_android_media_boundaries(current_index: Option<usize>, total_count: usize) {
        if let Some(index) = current_index {
            if total_count > 0 {
                let is_first = index == 0;
                let is_last = index == total_count - 1;

                // We need to get current metadata to update the media control
                if let Some(state_lock) = PLAYER_STATE.get() {
                    if let Ok(state) = state_lock.lock() {
                        if index < state.playlist.len() {
                            let music = state.playlist[index].metadata.clone();
                            let is_playing = CURRENT_STREAM.load(Ordering::SeqCst) != 0;
                            drop(state);

                            tauri::async_runtime::spawn(async move {
                                let handle = app_handle();
                                let image_path =
                                    match handle.fluyer().metadata_get_image(music.path.clone()) {
                                        Ok(res) => res.path,
                                        Err(_) => None,
                                    };

                                let _ = handle.fluyer().update_media_control(
                                    music.title.unwrap_or("Unknown".to_string()),
                                    music.artist.unwrap_or("Unknown".to_string()),
                                    music.album.unwrap_or("Unknown".to_string()),
                                    music.duration.unwrap_or(0) as u64,
                                    image_path,
                                    is_playing,
                                    is_first,
                                    is_last,
                                );
                            });
                        }
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

    fn play_next(from_user: bool) {
        tauri::async_runtime::spawn_blocking(move || {
            if let Some(state_lock) = PLAYER_STATE.get() {
                let next_index = {
                    let state = match state_lock.lock() {
                        Ok(s) => s,
                        Err(e) => {
                            crate::error!("Failed to lock player state: {}", e);
                            return;
                        }
                    };

                    // If from user, ignore verify RepeatOne (force change) - usually Next button skips even if RepeatOne
                    // But if RepeatOne is set, usually Next goes to next track, Auto-advance repeats.
                    // Assuming RepeatOne applies only to auto-advance.

                    match (state.current_index, state.repeat_mode) {
                        (Some(current), RepeatMode::One) if !from_user => Some(current),
                        (Some(current), _) if current + 1 < state.playlist.len() => {
                            Some(current + 1)
                        }
                        (Some(_), RepeatMode::All) => Some(0),
                        _ => None,
                    }
                };

                if let Some(index) = next_index {
                    // Get the next music metadata
                    let (music, total_count) = {
                        let state = match state_lock.lock() {
                            Ok(s) => s,
                            Err(e) => {
                                crate::error!("Failed to lock player state: {}", e);
                                return;
                            }
                        };
                        (state.playlist[index].metadata.clone(), state.playlist.len())
                    };

                    // For gapless playback, remove the old stream and load the new one without stopping the mixer
                    #[cfg(desktop)]
                    unsafe {
                        let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                        if current_stream != 0 {
                            BASS_Mixer_ChannelRemove(current_stream);
                            BASS_StreamFree(current_stream);
                            CURRENT_STREAM.store(0, Ordering::SeqCst);
                        }
                    }

                    #[cfg(target_os = "android")]
                    {
                        if let Some(bass) = bass_android::get_bass() {
                            unsafe {
                                let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                                if current_stream != 0 {
                                    (bass.bass_mixer_channel_remove)(current_stream);
                                    (bass.bass_stream_free)(current_stream);
                                    CURRENT_STREAM.store(0, Ordering::SeqCst);
                                }
                            }
                        }
                    }

                    if Self::load_music(music, index, total_count) {
                        if let Ok(mut state) = state_lock.lock() {
                            state.current_index = Some(index);
                        }
                        Self::emit_sync(true);
                    }
                } else if !from_user {
                    // Only stop if auto-advance and end of playlist
                    Self::stop_current_stream();
                    if let Ok(mut state) = state_lock.lock() {
                        state.current_index = None;
                    }
                }
                // If from_user and None, do nothing (don't stop)
            }
        });
    }

    fn play_previous() {
        tauri::async_runtime::spawn_blocking(|| {
            if let Some(state_lock) = PLAYER_STATE.get() {
                let prev_index = {
                    let state = match state_lock.lock() {
                        Ok(s) => s,
                        Err(e) => {
                            crate::error!("Failed to lock player state: {}", e);
                            return;
                        }
                    };

                    match (state.current_index, state.repeat_mode) {
                        (Some(current), _) if current > 0 => Some(current - 1),
                        (Some(_), RepeatMode::All) => Some(state.playlist.len() - 1),
                        _ => None,
                    }
                };

                if let Some(index) = prev_index {
                    // Get the next music metadata
                    let (music, total_count) = {
                        let state = match state_lock.lock() {
                            Ok(s) => s,
                            Err(e) => {
                                crate::error!("Failed to lock player state: {}", e);
                                return;
                            }
                        };
                        (state.playlist[index].metadata.clone(), state.playlist.len())
                    };

                    Self::stop_current_stream();

                    if Self::load_music(music, index, total_count) {
                        if let Ok(mut state) = state_lock.lock() {
                            state.current_index = Some(index);
                        }
                        Self::play_pause(true);
                        Self::emit_sync(true);
                    }
                }
                // If None (start of playlist and !RepeatAll), do nothing
            }
        });
    }

    fn load_music(music: MusicMetadata, index: usize, total_count: usize) -> bool {
        #[cfg(desktop)]
        unsafe {
            let path = CString::new(music.path.clone()).unwrap();
            let stream =
                BASS_StreamCreateFile(false, path.as_ptr() as *const _, 0, 0, BASS_STREAM_DECODE);

            if stream == 0 {
                let bass_error = BASS_ErrorGetCode();
                crate::warn!(
                    "BASS failed to load: {}, error: {}. Trying FFmpeg fallback...",
                    music.path,
                    bass_error
                );

                // Try FFmpeg fallback - convert to PCM WAV
                if let Some(wav_path) = Self::convert_to_pcm_wav(&music.path) {
                    let wav_cstring = CString::new(wav_path.to_string_lossy().as_ref()).unwrap();
                    let wav_stream = BASS_StreamCreateFile(
                        false,
                        wav_cstring.as_ptr() as *const _,
                        0,
                        0,
                        BASS_STREAM_DECODE,
                    );

                    if wav_stream != 0 {
                        let ok = BASS_Mixer_StreamAddChannel(
                            BASS_MIXER,
                            wav_stream,
                            BASS_MIXER_NORAMPIN,
                        );
                        if ok != 0 {
                            CURRENT_STREAM.store(wav_stream, Ordering::SeqCst);
                            // Store the temp WAV path for cleanup
                            if let Some(temp_path_lock) = TEMP_WAV_PATH.get() {
                                if let Ok(mut temp_path) = temp_path_lock.lock() {
                                    *temp_path = Some(wav_path.clone());
                                }
                            }
                            crate::info!("Successfully loaded via FFmpeg: {}", music.path);
                            return true;
                        } else {
                            crate::error!(
                                "Failed to add FFmpeg-converted channel to mixer: {}, error: {}",
                                music.path,
                                BASS_ErrorGetCode()
                            );
                            BASS_StreamFree(wav_stream);
                        }
                    } else {
                        crate::error!(
                            "BASS failed to load FFmpeg-converted WAV: {}, error: {}",
                            wav_path.display(),
                            BASS_ErrorGetCode()
                        );
                    }
                    // Clean up the temp file if loading failed
                    let _ = std::fs::remove_file(&wav_path);
                }

                crate::error!(
                    "Failed to load music (both BASS and FFmpeg failed): {}",
                    music.path
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

            CURRENT_STREAM.store(stream, Ordering::SeqCst);
            crate::info!("Successfully loaded: {}", music.path);

            #[cfg(target_os = "android")]
            {
                let music_clone = music.clone();
                tauri::async_runtime::spawn(async move {
                    let handle = app_handle();
                    let image_path =
                        match handle.fluyer().metadata_get_image(music_clone.path.clone()) {
                            Ok(res) => res.path,
                            Err(_) => None,
                        };

                    let _ = handle.fluyer().update_media_control(
                        music_clone.title.unwrap_or("Unknown".to_string()),
                        music_clone.artist.unwrap_or("Unknown".to_string()),
                        music_clone.album.unwrap_or("Unknown".to_string()),
                        music_clone.duration.unwrap_or(0) as u64,
                        image_path,
                        true,
                    );
                });
            }

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
                        let bass_error = (bass.bass_error_get_code)();
                        crate::warn!(
                            "BASS failed to load: {}, error: {}. Trying FFmpeg fallback...",
                            music.path,
                            bass_error
                        );

                        // Try FFmpeg fallback - convert to PCM WAV using FluyerPlugin
                        if let Some(wav_path) = Self::convert_to_pcm_wav_android(&music.path) {
                            let wav_cstring = CString::new(wav_path.as_str()).unwrap();
                            let wav_stream = (bass.bass_stream_create_file)(
                                false,
                                wav_cstring.as_ptr() as *const _,
                                0,
                                0,
                                BASS_STREAM_DECODE,
                            );

                            if wav_stream != 0 {
                                let ok = (bass.bass_mixer_stream_add_channel)(
                                    BASS_MIXER,
                                    wav_stream,
                                    BASS_MIXER_NORAMPIN,
                                );
                                if ok != 0 {
                                    CURRENT_STREAM.store(wav_stream, Ordering::SeqCst);
                                    // Store the temp WAV path for cleanup
                                    if let Some(temp_path_lock) = TEMP_WAV_PATH.get() {
                                        if let Ok(mut temp_path) = temp_path_lock.lock() {
                                            *temp_path = Some(PathBuf::from(&wav_path));
                                        }
                                    }
                                    crate::info!("Successfully loaded via FFmpeg: {}", music.path);
                                    return true;
                                } else {
                                    crate::error!(
                                        "Failed to add FFmpeg-converted channel to mixer: {}, error: {}",
                                        music.path,
                                        (bass.bass_error_get_code)()
                                    );
                                    (bass.bass_stream_free)(wav_stream);
                                }
                            } else {
                                crate::error!(
                                    "BASS failed to load FFmpeg-converted WAV: {}, error: {}",
                                    wav_path,
                                    (bass.bass_error_get_code)()
                                );
                            }
                            // Clean up the temp file if loading failed
                            let _ = std::fs::remove_file(&wav_path);
                        }

                        crate::error!(
                            "Failed to load music (both BASS and FFmpeg failed): {}",
                            music.path
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

                    CURRENT_STREAM.store(stream, Ordering::SeqCst);
                    crate::info!("Successfully loaded: {}", music.path);

                    #[cfg(target_os = "android")]
                    {
                        let music_clone = music.clone();
                        tauri::async_runtime::spawn(async move {
                            let handle = app_handle();
                            let image_path = match handle
                                .fluyer()
                                .metadata_get_image(music_clone.path.clone())
                            {
                                Ok(res) => res.path,
                                Err(_) => None,
                            };

                            let _ = handle.fluyer().update_media_control(
                                music_clone.title.unwrap_or("Unknown".to_string()),
                                music_clone.artist.unwrap_or("Unknown".to_string()),
                                music_clone.album.unwrap_or("Unknown".to_string()),
                                music_clone.duration.unwrap_or(0) as u64,
                                image_path,
                                true,
                                index == 0,
                                index == total_count - 1,
                            );
                        });
                    }

                    return true;
                }
            }
            false
        }
    }

    /// Convert audio file to PCM WAV using FFmpegKit on Android
    #[cfg(target_os = "android")]
    fn convert_to_pcm_wav_android(source_path: &str) -> Option<String> {
        crate::info!("Converting {} to PCM WAV via FFmpegKit...", source_path);

        match app_handle()
            .fluyer()
            .audio_convert_to_wav(source_path.to_string())
        {
            Ok(response) => {
                if let Some(path) = response.path {
                    crate::info!("Successfully converted to PCM WAV: {}", path);
                    Some(path)
                } else {
                    crate::error!("FFmpegKit conversion returned no path");
                    None
                }
            }
            Err(e) => {
                crate::error!("FFmpegKit conversion failed: {}", e);
                None
            }
        }
    }

    /// Convert audio file to PCM WAV using FFmpeg for BASS compatibility
    #[cfg(desktop)]
    fn convert_to_pcm_wav(source_path: &str) -> Option<PathBuf> {
        use std::process::Command;

        // Get FFmpeg path from resources
        let ffmpeg_path = {
            #[cfg(target_os = "linux")]
            {
                PathBuf::from("/usr/lib/fluyer/ffmpeg")
            }
            #[cfg(not(target_os = "linux"))]
            {
                app_handle()
                    .path()
                    .resource_dir()
                    .ok()?
                    .join("libs/ffmpeg/bin/ffmpeg")
            }
        };

        // Generate temp output path in app data directory
        let app_data_dir = app_handle().path().app_data_dir().ok()?;
        let temp_dir = app_data_dir.join("temp");
        std::fs::create_dir_all(&temp_dir).ok()?;

        // Create a unique filename based on the source file
        let source_file_name = std::path::Path::new(source_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("audio");
        let output_path = temp_dir.join(format!("{}_converted.wav", source_file_name));

        // Remove existing file if any
        let _ = std::fs::remove_file(&output_path);

        crate::info!("Converting {} to PCM WAV...", source_path);

        // Convert to PCM WAV (signed 16-bit little-endian, which BASS supports well)
        // Using pcm_s16le codec for fastest conversion
        let status = Command::new(&ffmpeg_path)
            .args(&[
                "-y", // Overwrite output
                "-i",
                source_path, // Input file
                "-vn",       // No video
                "-acodec",
                "pcm_s16le", // PCM signed 16-bit little-endian
                "-ar",
                "44100", // 44.1kHz sample rate (match BASS mixer)
                "-ac",
                "2", // Stereo
                "-f",
                "wav", // WAV format
            ])
            .arg(&output_path)
            .output();

        match status {
            Ok(output) if output.status.success() => {
                crate::info!(
                    "Successfully converted to PCM WAV: {}",
                    output_path.display()
                );
                Some(output_path)
            }
            Ok(output) => {
                crate::error!(
                    "FFmpeg conversion failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                None
            }
            Err(e) => {
                crate::error!("Failed to run FFmpeg: {}", e);
                None
            }
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
        // Emit sync event to notify frontend of the final backend state
        Self::emit_sync(false);
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
                        } else {
                            let _ = app_handle()
                                .fluyer()
                                .set_media_control_state(true, Self::get_current_duration() as u64);
                        }
                    } else {
                        if (bass.bass_channel_pause)(BASS_MIXER) == 0 {
                            crate::error!(
                                "Failed to pause, error: {}",
                                (bass.bass_error_get_code)()
                            );
                        } else {
                            let _ = app_handle().fluyer().set_media_control_state(
                                false,
                                Self::get_current_duration() as u64,
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
                    let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                    if current_stream != 0 {
                        let state = BASS_ChannelIsActive(current_stream);
                        if state == BASS_ACTIVE_STOPPED {
                            // Track ended, trigger next
                            crate::info!("Track ended, playing next");
                            Self::play_next(false);
                        }
                    }
                }

                #[cfg(target_os = "android")]
                {
                    if let Some(bass) = bass_android::get_bass() {
                        unsafe {
                            let current_stream = CURRENT_STREAM.load(Ordering::SeqCst);
                            if current_stream != 0 {
                                let state = (bass.bass_channel_is_active)(current_stream);
                                if state == BASS_ACTIVE_STOPPED {
                                    crate::info!("Track ended, playing next");
                                    Self::play_next(false);
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
