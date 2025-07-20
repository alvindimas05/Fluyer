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
pub fn get_log_path() -> String {
    format!("{}{}", temp_dir().display(), get_log_name())
}

#[allow(dead_code)]
pub fn get_mpv_log_path() -> String {
    format!("{}{}", temp_dir().display(), get_mpv_log_name())
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        if cfg!(debug_assertions) {
            if(crate::platform::is_mobile()){
                println!("{}", $msg);
            } else {
                log::debug!("{}", $msg);
            }
        }
    };
    ($msg:expr, $( $args:expr ),+ ) => {
        if cfg!(debug_assertions) {
            if(crate::platform::is_mobile()){
                println!($msg, $( $args ),+);
            } else {
                log::debug!($msg, $( $args ),+);
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if(crate::platform::is_mobile()){
            eprintln!("{}", $msg);
        } else {
            log::error!("{}", $msg);
        }
    };
    ($msg:expr, $( $args:expr ),+ ) => {
        if(crate::platform::is_mobile()){
            eprintln!($msg, $( $args ),+);
        } else {
            log::error!($msg, $( $args ),+);
        }
    };
}

#[allow(unused_imports)]
pub(crate) use debug;
#[allow(unused_imports)]
pub(crate) use error;
