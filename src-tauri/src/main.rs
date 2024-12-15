// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::{env::temp_dir, fs::File};

fn main() {
    init_log();
    fluyer_lib::run()
}

fn init_log() {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            File::create(format!("{}/fluyer.log", temp_dir().display().to_string())).unwrap(),
        ),
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])
    .unwrap();
}
