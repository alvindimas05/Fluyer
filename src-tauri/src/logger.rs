use std::env::temp_dir;

/// Initialize logging system for the application
#[cfg(desktop)]
pub fn init() {
    use simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, TermLogger,
        TerminalMode, WriteLogger,
    };
    use std::fs::File;
    
    let log_path = get_log_path();

    let config = ConfigBuilder::new()
        .add_filter_ignore_str("symphonia_core")
        .build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Debug,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    // Add file logger if file creation succeeds
    if let Ok(log_file) = File::create(&log_path) {
        loggers.push(WriteLogger::new(LevelFilter::Debug, config, log_file));
    }

    CombinedLogger::init(loggers).unwrap();
    log::debug!("The log file is saved at {}", log_path);
}

#[allow(dead_code)]
pub fn get_log_name() -> String {
    "fluyer.log".to_string()
}

#[allow(dead_code)]
pub fn get_mpv_log_name() -> String {
    "fluyer-mpv.log".to_string()
}

#[allow(dead_code)]
#[allow(dead_code)]
pub fn get_log_path() -> String {
    temp_dir().join(get_log_name()).display().to_string()
}

#[allow(dead_code)]
pub fn get_mpv_log_path() -> String {
    temp_dir().join(get_mpv_log_name()).display().to_string()
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::debug!("{}", $msg);
        } else {
            println!("{}", $msg);
        }
    };
    ($msg:expr, $( $args:expr ),+ ) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::debug!($msg, $( $args ),+);
        } else {
            println!($msg, $( $args ),+);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::info!("{}", $msg);
        } else {
            println!("{}", $msg);
        }
    };
    ($msg:expr, $( $args:expr ),+ ) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::info!($msg, $( $args ),+);
        } else {
            println!($msg, $( $args ),+);
        }
    };
}

#[macro_export]
macro_rules! warn_ {
    ($msg:expr) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::warn!("{}", $msg);
        } else {
            println!("{}", $msg);
        }
    };
    ($msg:expr, $( $args:expr ),+ ) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::warn!($msg, $( $args ),+);
        } else {
            println!($msg, $( $args ),+);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::error!("{}", $msg);
        } else {
            println!("{}", $msg);
        }
    };
    ($msg:expr, $( $args:expr ),+ ) => {
        if cfg!(debug_assertions) || crate::platform::is_desktop() {
            log::error!($msg, $( $args ),+);
        } else {
            println!($msg, $( $args ),+);
        }
    };
}

#[allow(unused_imports)]
pub(crate) use {debug, error, info, warn_ as warn};
