//! Aspect ratio modes.

use serde::{Deserialize, Serialize};

/// Aspect ratio mode for the playfield.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectRatioMode {
    /// Automatic based on window size.
    Auto,
    /// Force 16:9 aspect ratio.
    Ratio16_9,
    /// Force 4:3 aspect ratio.
    Ratio4_3,
}

impl Default for AspectRatioMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for AspectRatioMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Ratio16_9 => write!(f, "16:9"),
            Self::Ratio4_3 => write!(f, "4:3"),
        }
    }
}
