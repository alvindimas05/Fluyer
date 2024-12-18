// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::{env::temp_dir, fs::File};

fn main() {
    init_logging();
    init_android_logging();
    fluyer_lib::run()
}

fn init_logging() {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create(format!("{}/fluyer.log", temp_dir().display().to_string())).unwrap(),
        ),
        TermLogger::new(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Warn
            },
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])
    .unwrap();
}

fn init_android_logging() {
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("{{app.name}}"),
        );
    }
}
