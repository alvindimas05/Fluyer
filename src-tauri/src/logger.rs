use std::env::temp_dir;

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
