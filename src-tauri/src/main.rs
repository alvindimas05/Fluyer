// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::{env::temp_dir, fs::File};

fn main() {
    #[cfg(desktop)]
    {
        init_logging();
    }
    fluyer_lib::run()
}

fn init_logging() {
    println!("The log file is saved at {}", temp_dir().display());
    CombinedLogger::init(vec![
        WriteLogger::new(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Warn
            },
            Config::default(),
            File::create(format!("{}/fluyer.log", temp_dir().display())).unwrap(),
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
