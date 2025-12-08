pub mod directory;
pub mod metadata;
pub mod player;
pub mod playlist;
pub mod visualizer;

// Re-export all command functions for easy access
pub use directory::*;
pub use metadata::*;
pub use player::*;
pub use playlist::*;
pub use visualizer::*;
