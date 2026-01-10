use std::env::temp_dir;

use crate::state::app_handle;
use log::{Log, Metadata, Record};
use tauri::Emitter;

struct PrintLogger;

/// Get ANSI color code for log level
fn level_color(level: log::Level) -> &'static str {
    match level {
        log::Level::Error => "\x1b[31m", // Red
        log::Level::Warn => "\x1b[33m",  // Yellow
        log::Level::Info => "\x1b[32m",  // Green
        log::Level::Debug => "\x1b[36m", // Cyan
        log::Level::Trace => "\x1b[37m", // White
    }
}

const RESET: &str = "\x1b[0m";

impl Log for PrintLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::LevelFilter::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = level_color(record.level());
        println!(
            "{}[{}]{} {} - {}",
            color,
            record.level(),
            RESET,
            record.target(),
            record.args()
        );
        app_handle()
            .emit(
                crate::commands::route::LOG,
                format!(
                    "[{}] {} - {}",
                    record.level(),
                    record.target(),
                    record.args()
                ),
            )
            .expect(format!("Failed to emit {}", crate::commands::route::LOG).as_str());
    }

    fn flush(&self) {}
}

static LOGGER: PrintLogger = PrintLogger;

/// Initialize logging system for the application
pub fn init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Failed to set logger");
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
pub fn get_log_path() -> String {
    temp_dir().join(get_log_name()).display().to_string()
}

#[allow(dead_code)]
pub fn get_mpv_log_path() -> String {
    temp_dir().join(get_mpv_log_name()).display().to_string()
}
