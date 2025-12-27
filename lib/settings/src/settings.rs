//! Main settings structure.

use crate::{AspectRatioMode, HitWindowMode, default_keybinds};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Settings file name.
pub const SETTINGS_FILE: &str = "settings.toml";

/// Persistent user settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    /// Master volume (0.0 to 1.0).
    pub master_volume: f32,
    /// Scroll speed in milliseconds.
    pub scroll_speed: f64,
    /// Global audio offset in milliseconds.
    /// Positive = notes appear later, Negative = notes appear earlier.
    #[serde(default)]
    pub global_audio_offset_ms: f64,
    /// Hit window calculation mode.
    pub hit_window_mode: HitWindowMode,
    /// Hit window value (OD or judge level).
    pub hit_window_value: f64,
    /// Aspect ratio mode.
    pub aspect_ratio_mode: AspectRatioMode,
    /// Current skin name.
    pub current_skin: String,
    /// Keybinds per key count.
    pub keybinds: HashMap<String, Vec<String>>,
}

impl GameSettings {
    /// Creates default settings.
    pub fn new() -> Self {
        Self {
            master_volume: 0.5,
            scroll_speed: 500.0,
            global_audio_offset_ms: 0.0,
            hit_window_mode: HitWindowMode::OsuOD,
            hit_window_value: 5.0,
            aspect_ratio_mode: AspectRatioMode::Auto,
            current_skin: "default".to_string(),
            keybinds: default_keybinds(),
        }
    }

    /// Loads settings from a file, or returns defaults if not found.
    pub fn load_from<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(content) = fs::read_to_string(path.as_ref()) {
            if let Ok(settings) = toml::from_str::<GameSettings>(&content) {
                return settings;
            }
            eprintln!("Failed to parse settings file, using defaults.");
        }
        Self::new()
    }

    /// Loads settings from the default file.
    pub fn load() -> Self {
        Self::load_from(SETTINGS_FILE)
    }

    /// Saves settings to a file.
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }

    /// Saves settings to the default file.
    pub fn save(&self) -> Result<(), std::io::Error> {
        self.save_to(SETTINGS_FILE)
    }

    /// Gets keybinds for a specific key count.
    pub fn get_keybinds(&self, key_count: usize) -> Option<&Vec<String>> {
        self.keybinds.get(&key_count.to_string())
    }

    /// Sets keybinds for a specific key count.
    pub fn set_keybinds(&mut self, key_count: usize, keys: Vec<String>) {
        self.keybinds.insert(key_count.to_string(), keys);
    }

    /// Resets keybinds to defaults.
    pub fn reset_keybinds(&mut self) {
        self.keybinds = default_keybinds();
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self::new()
    }
}
