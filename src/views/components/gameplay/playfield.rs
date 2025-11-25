use crate::models::engine::{
    HIT_LINE_Y, InstanceRaw, NUM_COLUMNS, NoteData, PixelSystem, PlayfieldConfig, VISIBLE_DISTANCE,
};

pub struct PlayfieldDisplay {
    pub config: PlayfieldConfig,
}

impl PlayfieldDisplay {
    pub fn new(config: PlayfieldConfig) -> Self {
        Self { config }
    }

    pub fn get_bounds(&self, pixel_system: &PixelSystem) -> (f32, f32) {
        let width = pixel_system
            .x_pixels_to_normalized(self.config.column_width_pixels * NUM_COLUMNS as f32);
        let x = -width / 2.0;
        (x, width)
    }

    pub fn render_notes(
        &self,
        visible_notes: &[NoteData],
        song_time: f64,
        scroll_speed_ms: f64,
        pixel_system: &PixelSystem,
    ) -> Vec<(usize, InstanceRaw)> {
        let (playfield_x, _playfield_width) = self.get_bounds(pixel_system);

        let column_width_norm =
            pixel_system.x_pixels_to_normalized(self.config.column_width_pixels);
        let note_width_norm = pixel_system.x_pixels_to_normalized(self.config.note_width_pixels);
        let note_height_norm = pixel_system.y_pixels_to_normalized(self.config.note_height_pixels);

        let mut instances = Vec::with_capacity(visible_notes.len());

        for note in visible_notes {
            if note.hit {
                continue;
            }

            let time_to_hit = note.timestamp_ms - song_time;
            let progress = time_to_hit / scroll_speed_ms;
            let y_pos = HIT_LINE_Y + (VISIBLE_DISTANCE * progress as f32);

            let center_x =
                playfield_x + (note.column as f32 * column_width_norm) + (column_width_norm / 2.0);

            instances.push((
                note.column,
                InstanceRaw {
                    offset: [center_x, y_pos],
                    scale: [note_width_norm, note_height_norm],
                },
            ));
        }

        instances
    }

    pub fn render_receptors(&self, pixel_system: &PixelSystem) -> Vec<InstanceRaw> {
        let (playfield_x, _playfield_width) = self.get_bounds(pixel_system);

        let column_width_norm =
            pixel_system.x_pixels_to_normalized(self.config.column_width_pixels);
        let receptor_width_norm =
            pixel_system.x_pixels_to_normalized(self.config.note_width_pixels);

        // CORRECTION ICI : On utilise note_width_pixels pour la hauteur aussi
        // Cela rend le récepteur carré (90x90) au lieu d'un rectangle plat (90x20)
        let receptor_height_norm =
            pixel_system.y_pixels_to_normalized(self.config.note_width_pixels);

        let mut instances = Vec::with_capacity(NUM_COLUMNS);

        for col in 0..NUM_COLUMNS {
            let center_x =
                playfield_x + (col as f32 * column_width_norm) + (column_width_norm / 2.0);

            instances.push(InstanceRaw {
                offset: [center_x, HIT_LINE_Y],
                scale: [receptor_width_norm, receptor_height_norm],
            });
        }

        instances
    }
}
