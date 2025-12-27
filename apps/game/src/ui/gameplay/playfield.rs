//! Playfield - orchestrates all columns for gameplay rendering.

use std::sync::Arc;

use crate::graphics::assets::{ColumnAssets, SkinAssets};
use crate::graphics::primitives::InstanceRaw;
use engine::NoteData;

use super::column::Column;

/// Hit line Y position in normalized coordinates.
pub const HIT_LINE_Y: f32 = -0.8;
/// Spawn Y position in normalized coordinates.
pub const SPAWN_Y: f32 = 1.2;
/// Visible distance from spawn to hit line.
pub const VISIBLE_DISTANCE: f32 = SPAWN_Y - HIT_LINE_Y;

/// Playfield configuration.
#[derive(Clone)]
pub struct PlayfieldConfig {
    pub column_width: f32,
    pub note_width: f32,
    pub note_height: f32,
    pub receptor_width: f32,
    pub receptor_height: f32,
    pub spacing: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}

impl Default for PlayfieldConfig {
    fn default() -> Self {
        Self {
            column_width: 0.1,
            note_width: 0.09,
            note_height: 0.05,
            receptor_width: 0.09,
            receptor_height: 0.05,
            spacing: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}

/// The playfield containing all columns.
pub struct Playfield {
    columns: Vec<Column>,
    config: PlayfieldConfig,
}

impl Playfield {
    /// Create a new empty playfield.
    pub fn new(config: PlayfieldConfig) -> Self {
        Self {
            columns: Vec::new(),
            config,
        }
    }

    /// Initialize columns from skin assets.
    pub fn init_from_assets(&mut self, assets: &SkinAssets) {
        self.columns.clear();
        for (i, col_assets) in assets.columns().iter().enumerate() {
            self.columns.push(Column::new(
                i,
                Arc::new(ColumnAssets {
                    note: Arc::clone(&col_assets.note),
                    receptor: Arc::clone(&col_assets.receptor),
                    receptor_pressed: Arc::clone(&col_assets.receptor_pressed),
                }),
            ));
        }
    }

    /// Get the number of columns.
    pub fn key_count(&self) -> usize {
        self.columns.len()
    }

    /// Get a column by index.
    pub fn column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Get a mutable column by index.
    pub fn column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Get all columns.
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Get mutable reference to all columns.
    pub fn columns_mut(&mut self) -> &mut [Column] {
        &mut self.columns
    }

    /// Calculate total playfield width in normalized coordinates.
    pub fn total_width(&self) -> f32 {
        let cols = self.columns.len() as f32;
        if cols == 0.0 {
            return 0.0;
        }
        let spaces = (cols - 1.0).max(0.0);
        (cols * self.config.column_width) + (spaces * self.config.spacing)
    }

    /// Calculate playfield left X position (centered).
    pub fn left_x(&self) -> f32 {
        -self.total_width() / 2.0 + self.config.x_offset
    }

    /// Clear all column instances for new frame.
    pub fn clear_instances(&mut self) {
        for col in &mut self.columns {
            col.clear_instances();
        }
    }

    /// Render all visible notes.
    pub fn render_notes(
        &mut self,
        visible_notes: &[NoteData],
        song_time_ms: f64,
        scroll_speed_ms: f64,
    ) {
        self.clear_instances();

        let left_x = self.left_x();
        let hit_line_y = HIT_LINE_Y + self.config.y_offset;

        for note in visible_notes {
            let col_idx = note.column();
            if let Some(col) = self.columns.get_mut(col_idx) {
                col.render_note(
                    note,
                    song_time_ms,
                    scroll_speed_ms,
                    left_x,
                    self.config.column_width,
                    self.config.spacing,
                    self.config.note_width,
                    self.config.note_height,
                    hit_line_y,
                    VISIBLE_DISTANCE,
                );
            }
        }
    }

    /// Get all receptor instances.
    pub fn receptor_instances(&self) -> Vec<InstanceRaw> {
        let left_x = self.left_x();
        let hit_line_y = HIT_LINE_Y + self.config.y_offset;

        self.columns
            .iter()
            .map(|col| {
                col.render_receptor(
                    left_x,
                    self.config.column_width,
                    self.config.spacing,
                    self.config.receptor_width,
                    self.config.receptor_height,
                    hit_line_y,
                )
            })
            .collect()
    }

    /*
        /// Collect all note instances grouped by visual type.
        pub fn collect_instances(&self) -> NoteInstancesByType {
            let mut result = NoteInstancesByType::default();

            for col in &self.columns {
                for inst in col.note_instances() {
                    match inst.visual {
                        NoteVisual::Tap => result.taps.push((col.index, inst.instance)),
                        NoteVisual::Mine => result.mines.push(inst.instance),
                        NoteVisual::HoldBody => result.hold_bodies.push(inst.instance),
                        NoteVisual::HoldEnd => result.hold_ends.push(inst.instance),
                        NoteVisual::BurstBody => result.burst_bodies.push(inst.instance),
                        NoteVisual::BurstEnd => result.burst_ends.push(inst.instance),
                    }
                }
            }

            result
        }
    */

    /*
        /// Update config from pixel system.
        pub fn update_from_pixels(
            &mut self,
            pixel_system: &PixelSystem,
            pixels: &PlayfieldPixelConfig,
        ) {
            self.config.column_width = pixel_system.x_pixels_to_normalized(pixels.column_width);
            self.config.note_width = pixel_system.x_pixels_to_normalized(pixels.note_width);
            self.config.note_height = pixel_system.y_pixels_to_normalized(pixels.note_height);
            self.config.receptor_width = pixel_system.x_pixels_to_normalized(pixels.receptor_width);
            self.config.receptor_height = pixel_system.y_pixels_to_normalized(pixels.receptor_height);
            self.config.spacing = pixel_system.x_pixels_to_normalized(pixels.spacing);
            self.config.x_offset = pixel_system.x_pixels_to_normalized(pixels.x_offset);
            self.config.y_offset = pixel_system.y_pixels_to_normalized(pixels.y_offset);
        }
    */
}

/*
/// Pixel-based playfield config (for skin settings).
#[derive(Clone)]
pub struct PlayfieldPixelConfig {
    pub column_width: f32,
    pub note_width: f32,
    pub note_height: f32,
    pub receptor_width: f32,
    pub receptor_height: f32,
    pub spacing: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}

impl Default for PlayfieldPixelConfig {
    fn default() -> Self {
        Self {
            column_width: 100.0,
            note_width: 90.0,
            note_height: 90.0,
            receptor_width: 90.0,
            receptor_height: 90.0,
            spacing: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}
*/

/*
/// Note instances grouped by visual type for efficient batch rendering.
#[derive(Default)]
pub struct NoteInstancesByType {
    /// Tap notes with column index
    pub taps: Vec<(usize, InstanceRaw)>,
    pub mines: Vec<InstanceRaw>,
    pub hold_bodies: Vec<InstanceRaw>,
    pub hold_ends: Vec<InstanceRaw>,
    pub burst_bodies: Vec<InstanceRaw>,
    pub burst_ends: Vec<InstanceRaw>,
}
*/
