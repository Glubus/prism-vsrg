//! Prism Design System color palette and theme constants.
//!
//! These colors match the backend's prism.css design system.

use egui::Color32;

/// Primary accent color - crimson red
pub const PRISM_PRIMARY: Color32 = Color32::from_rgb(255, 0, 60);

/// Primary hover state
pub const PRISM_PRIMARY_HOVER: Color32 = Color32::from_rgb(255, 77, 106);

/// Background - near black
pub const PRISM_BG: Color32 = Color32::from_rgb(5, 5, 5);

/// Solid panel background
pub const PRISM_PANEL_SOLID: Color32 = Color32::from_rgb(10, 10, 10);

/// Main text color
pub const PRISM_TEXT: Color32 = Color32::WHITE;

/// Muted text color
pub const PRISM_TEXT_MUTED: Color32 = Color32::from_rgb(136, 136, 136);

/// Subtle text color
pub const PRISM_TEXT_SUBTLE: Color32 = Color32::from_rgb(102, 102, 102);

/// Border color
pub const PRISM_BORDER: Color32 = Color32::from_rgb(51, 51, 51);

/// Success color - green
pub const PRISM_SUCCESS: Color32 = Color32::from_rgb(0, 255, 136);

/// Error color - same as primary
pub const PRISM_ERROR: Color32 = PRISM_PRIMARY;

/// Panel background with transparency (runtime helper)
#[inline]
pub fn prism_panel() -> Color32 {
    Color32::from_rgba_unmultiplied(40, 0, 10, 153)
}

/// Primary glow color for shaders
#[inline]
pub fn prism_primary_glow(alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(255, 0, 60, alpha)
}

// ============================================================================
// WGPU color constants (as f32 arrays for shaders)
// ============================================================================

/// Primary color for shaders [R, G, B, A]
pub const PRISM_PRIMARY_F32: [f32; 4] = [1.0, 0.0, 0.235, 1.0];

/// Background color for shaders
pub const PRISM_BG_F32: [f32; 4] = [0.02, 0.02, 0.02, 1.0];
