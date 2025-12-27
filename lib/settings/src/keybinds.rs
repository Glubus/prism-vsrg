//! Keybind configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Default keybinds for 4K, 5K, 6K, and 7K.
pub fn default_keybinds() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    map.insert(
        "4".to_string(),
        vec![
            "KeyD".to_string(),
            "KeyF".to_string(),
            "KeyJ".to_string(),
            "KeyK".to_string(),
        ],
    );
    map.insert(
        "5".to_string(),
        vec![
            "KeyD".to_string(),
            "KeyF".to_string(),
            "Space".to_string(),
            "KeyJ".to_string(),
            "KeyK".to_string(),
        ],
    );
    map.insert(
        "6".to_string(),
        vec![
            "KeyS".to_string(),
            "KeyD".to_string(),
            "KeyF".to_string(),
            "KeyJ".to_string(),
            "KeyK".to_string(),
            "KeyL".to_string(),
        ],
    );
    map.insert(
        "7".to_string(),
        vec![
            "KeyS".to_string(),
            "KeyD".to_string(),
            "KeyF".to_string(),
            "Space".to_string(),
            "KeyJ".to_string(),
            "KeyK".to_string(),
            "KeyL".to_string(),
        ],
    );
    map
}

/// Keybind configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinds {
    /// Keybinds per key count (key = "4", "5", etc.)
    pub bindings: HashMap<String, Vec<String>>,
}

impl Keybinds {
    pub fn new() -> Self {
        Self {
            bindings: default_keybinds(),
        }
    }

    /// Get keybinds for a specific key count.
    pub fn get(&self, key_count: usize) -> Option<&Vec<String>> {
        self.bindings.get(&key_count.to_string())
    }

    /// Set keybinds for a specific key count.
    pub fn set(&mut self, key_count: usize, keys: Vec<String>) {
        self.bindings.insert(key_count.to_string(), keys);
    }

    /// Reset to defaults.
    pub fn reset(&mut self) {
        self.bindings = default_keybinds();
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self::new()
    }
}
