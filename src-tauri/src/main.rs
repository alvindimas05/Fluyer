// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod platform;

use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

fn main() {
    #[cfg(target_os = "linux")]
    fix_linux_webview_config();

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
            LevelFilter::Debug,
            config,
            log_file.unwrap(),
        ));
    }

    CombinedLogger::init(logs).unwrap();
    logger::debug!("The log file is saved at {}", log_path);
}

#[cfg(target_os = "linux")]
fn fix_linux_webview_config(){

    // multiple environment variable combinations for chromium webview
    std::env::set_var("WRY_WEBVIEW_BACKEND", "chromium");
    std::env::set_var("TAURI_WEBVIEW_BACKEND", "chromium");
    std::env::set_var("WEBVIEW_BACKEND", "chromium");

    // try to disable compositing and use hardware acceleration
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "0");
    std::env::set_var("WEBKIT_FORCE_HARDWARE_ACCELERATION", "1");

    // flags that might help with chromium detection
    std::env::set_var("CHROME_DEVEL_SANDBOX", "/usr/lib/chromium-browser/chrome-sandbox");
    std::env::set_var("CHROME_WRAPPER", "/usr/bin/chromium-browser");
}