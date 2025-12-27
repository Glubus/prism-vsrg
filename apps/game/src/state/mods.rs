//! Game modifiers system.
//!
//! This module defines gameplay mods that alter note behavior or visual effects.

use std::collections::HashSet;

/// Available gameplay modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameMod {
    /// Converts LN and burst to tap notes, removes mines.
    NoSpecial,
    /// Black overlay from bottom that grows with combo.
    Hidden,
    /// Screen hidden except for a thin horizontal strip.
    Flashlight,
    /// Notes visually rotate on themselves.
    Spinner,
}

impl GameMod {
    /// Returns a user-friendly display name for the mod.
    pub fn display_name(&self) -> &'static str {
        match self {
            GameMod::NoSpecial => "NO SPECIAL",
            GameMod::Hidden => "HIDDEN",
            GameMod::Flashlight => "FLASHLIGHT",
            GameMod::Spinner => "SPINNER",
        }
    }

    /// Returns a short description of what the mod does.
    pub fn description(&self) -> &'static str {
        match self {
            GameMod::NoSpecial => "Replaces LN/burst with taps, removes mines",
            GameMod::Hidden => "Screen darkens from bottom as combo grows",
            GameMod::Flashlight => "Only a thin strip is visible",
            GameMod::Spinner => "Notes rotate visually",
        }
    }

    /// Returns all available mods.
    pub fn all() -> &'static [GameMod] {
        &[
            GameMod::NoSpecial,
            GameMod::Hidden,
            GameMod::Flashlight,
            GameMod::Spinner,
        ]
    }
}

/// Tracks which gameplay mods are currently active.
#[derive(Clone, Debug, Default)]
pub struct ActiveMods {
    mods: HashSet<GameMod>,
}

impl ActiveMods {
    /// Creates a new empty set of active mods.
    pub fn new() -> Self {
        Self {
            mods: HashSet::new(),
        }
    }

    /// Checks if a specific mod is active.
    pub fn has(&self, m: GameMod) -> bool {
        self.mods.contains(&m)
    }

    /// Toggles a mod on or off.
    pub fn toggle(&mut self, m: GameMod) {
        if self.mods.contains(&m) {
            self.mods.remove(&m);
        } else {
            self.mods.insert(m);
        }
    }

    /// Returns an iterator over all active mods.
    pub fn iter(&self) -> impl Iterator<Item = &GameMod> {
        self.mods.iter()
    }

    /// Returns true if no mods are active.
    pub fn is_empty(&self) -> bool {
        self.mods.is_empty()
    }

    /// Clears all active mods.
    pub fn clear(&mut self) {
        self.mods.clear();
    }
}
