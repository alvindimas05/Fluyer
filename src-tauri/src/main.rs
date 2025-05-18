// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use simplelog::{
    ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::{env::temp_dir, fs::File};

fn main() {
    #[cfg(desktop)]
    init_logging();
    dotenvy::dotenv().ok();
    fluyer_lib::run()
}

#[cfg(desktop)]
fn init_logging() {
    use fluyer_lib::debug;
    use simplelog::ConfigBuilder;
    let log_path = format!("{}/fluyer.log", temp_dir().display());
    
    let mut config = ConfigBuilder::new();
    config.add_filter_ignore_str("symphonia_core");
    let config = config.build();

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Warn,
            config.clone(),
            File::create(log_path.clone()).unwrap(),
        ),
        TermLogger::new(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Warn
            },
            config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
    ]).unwrap();
    println!("The log file is saved at {}", log_path);
}