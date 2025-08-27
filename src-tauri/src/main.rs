// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod platform;

use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

fn main() {
    #[cfg(desktop)]
    init_logging();
    dotenvy::dotenv().ok();
    fluyer_lib::run()
}

#[cfg(desktop)]
fn init_logging() {
    use simplelog::{ConfigBuilder, SharedLogger};
    let log_path = logger::get_log_path();

    let mut config = ConfigBuilder::new();
    config.add_filter_ignore_str("symphonia_core");
    let config = config.build();

    let mut logs: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Debug,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    let log_file = File::create(log_path.clone());
    if log_file.is_ok() {
        logs.push(WriteLogger::new(
            LevelFilter::Warn,
            config,
            log_file.unwrap(),
        ));
    }

    CombinedLogger::init(logs).unwrap();
    logger::debug!("The log file is saved at {}", log_path);
}
