//! Settings - User configuration for Prism.
//!
//! # Modules
//!
//! - [`settings`] - Main GameSettings struct
//! - [`hit_window_mode`] - Hit window calculation modes
//! - [`aspect_ratio`] - Aspect ratio options
//! - [`keybinds`] - Keybind configuration

mod aspect_ratio;
mod hit_window_mode;
mod keybinds;
mod settings;

pub use aspect_ratio::AspectRatioMode;
pub use hit_window_mode::HitWindowMode;
pub use keybinds::{default_keybinds, Keybinds};
pub use settings::{GameSettings, SETTINGS_FILE};
