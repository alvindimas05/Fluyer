use simplelog::{CombinedLogger, ConfigBuilder, SharedLogger, TermLogger};

/// Initialize logging configuration
pub fn init_logging() {
    let config = ConfigBuilder::new().build();

    let logs: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        simplelog::LevelFilter::Debug,
        config,
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )];

    CombinedLogger::init(logs).expect("Failed to initialize logger");
}
