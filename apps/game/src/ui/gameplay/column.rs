//! Column - a single gameplay column with rendering logic.

use std::sync::Arc;
use wgpu::BindGroup;

use crate::graphics::assets::ColumnAssets;
use crate::graphics::primitives::InstanceRaw;
use engine::{NoteData, US_PER_MS};

/// Visual type of a rendered note.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoteVisual {
    Tap,
    Mine,
    HoldBody,
    HoldEnd,
    BurstBody,
    BurstEnd,
}

/// A renderable note instance.
pub struct NoteInstance {
    pub visual: NoteVisual,
    pub instance: InstanceRaw,
}

/// A single gameplay column.
pub struct Column {
    /// Column index (0-based)
    pub index: usize,
    /// Assets for this column
    assets: Arc<ColumnAssets>,
    /// Cached instances for this frame
    note_instances: Vec<NoteInstance>,
}

impl Column {
    pub fn new(index: usize, assets: Arc<ColumnAssets>) -> Self {
        Self {
            index,
            assets,
            note_instances: Vec::with_capacity(100),
        }
    }

    /// Get the note bind group for this column.
    pub fn note_bind_group(&self) -> &BindGroup {
        &self.assets.note
    }

    /// Get the appropriate receptor bind group.
    pub fn receptor_bind_group(&self, is_pressed: bool) -> &BindGroup {
        if is_pressed {
            &self.assets.receptor_pressed
        } else {
            &self.assets.receptor
        }
    }

    /// Compute X center position for this column.
    pub fn compute_center_x(&self, playfield_left: f32, column_width: f32, spacing: f32) -> f32 {
        let offset = self.index as f32 * (column_width + spacing);
        playfield_left + offset + (column_width / 2.0)
    }

    /// Render receptor instance for this column.
    pub fn render_receptor(
        &self,
        playfield_left: f32,
        column_width: f32,
        spacing: f32,
        receptor_width: f32,
        receptor_height: f32,
        hit_line_y: f32,
    ) -> InstanceRaw {
        let center_x = self.compute_center_x(playfield_left, column_width, spacing);
        InstanceRaw {
            offset: [center_x, hit_line_y],
            scale: [receptor_width, receptor_height],
        }
    }

    /// Clear note instances for new frame.
    pub fn clear_instances(&mut self) {
        self.note_instances.clear();
    }

    /// Render a note and add to instances.
    pub fn render_note(
        &mut self,
        note: &NoteData,
        song_time_ms: f64,
        scroll_speed_ms: f64,
        playfield_left: f32,
        column_width: f32,
        spacing: f32,
        note_width: f32,
        note_height: f32,
        hit_line_y: f32,
        visible_distance: f32,
    ) {
        if note.state.hit {
            return;
        }

        let center_x = self.compute_center_x(playfield_left, column_width, spacing);
        let note_time_ms = note.time_us() as f64 / US_PER_MS as f64;
        let time_to_hit = note_time_ms - song_time_ms;
        let progress = time_to_hit / scroll_speed_ms;
        let y_pos = hit_line_y + (visible_distance * progress as f32);

        let ln_width = note_width * 0.95;

        if note.is_tap() {
            self.note_instances.push(NoteInstance {
                visual: NoteVisual::Tap,
                instance: InstanceRaw {
                    offset: [center_x, y_pos],
                    scale: [note_width, note_height],
                },
            });
        } else if note.is_mine() {
            self.note_instances.push(NoteInstance {
                visual: NoteVisual::Mine,
                instance: InstanceRaw {
                    offset: [center_x, y_pos],
                    scale: [note_width, note_height],
                },
            });
        } else if note.is_hold() {
            self.render_long_note(
                note,
                note_time_ms,
                song_time_ms,
                scroll_speed_ms,
                center_x,
                y_pos,
                note_width,
                note_height,
                ln_width,
                hit_line_y,
                visible_distance,
                NoteVisual::HoldBody,
                NoteVisual::HoldEnd,
                note.state.hold.is_held,
            );
        } else if note.is_burst() {
            self.render_long_note(
                note,
                note_time_ms,
                song_time_ms,
                scroll_speed_ms,
                center_x,
                y_pos,
                note_width,
                note_height,
                ln_width,
                hit_line_y,
                visible_distance,
                NoteVisual::BurstBody,
                NoteVisual::BurstEnd,
                note.state.burst.current_hits > 0,
            );
        }
    }

    fn render_long_note(
        &mut self,
        note: &NoteData,
        note_time_ms: f64,
        song_time_ms: f64,
        scroll_speed_ms: f64,
        center_x: f32,
        y_pos: f32,
        note_width: f32,
        note_height: f32,
        ln_width: f32,
        hit_line_y: f32,
        visible_distance: f32,
        body_visual: NoteVisual,
        end_visual: NoteVisual,
        is_active: bool,
    ) {
        let note_duration_ms = note.duration_us() as f64 / US_PER_MS as f64;
        let end_time_ms = note_time_ms + note_duration_ms;
        let end_progress = (end_time_ms - song_time_ms) / scroll_speed_ms;
        let end_y = hit_line_y + (visible_distance * end_progress as f32);

        let clamped_y = if is_active && y_pos < hit_line_y {
            hit_line_y
        } else {
            y_pos
        };

        let body_height = (end_y - clamped_y).abs();
        let body_center_y = (clamped_y + end_y) / 2.0;

        // Body
        if body_height > 0.001 {
            self.note_instances.push(NoteInstance {
                visual: body_visual,
                instance: InstanceRaw {
                    offset: [center_x, body_center_y],
                    scale: [ln_width, body_height],
                },
            });
        }

        // Head (only if not clamped)
        if (clamped_y - y_pos).abs() < 0.001 {
            self.note_instances.push(NoteInstance {
                visual: NoteVisual::Tap,
                instance: InstanceRaw {
                    offset: [center_x, y_pos],
                    scale: [note_width, note_height],
                },
            });
        }

        // End
        self.note_instances.push(NoteInstance {
            visual: end_visual,
            instance: InstanceRaw {
                offset: [center_x, end_y],
                scale: [ln_width, note_height],
            },
        });
    }

    /// Get rendered note instances.
    pub fn note_instances(&self) -> &[NoteInstance] {
        &self.note_instances
    }
}
