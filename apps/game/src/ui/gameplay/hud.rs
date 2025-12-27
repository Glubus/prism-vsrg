//! HUD components for gameplay display.

use wgpu_text::glyph_brush::{Section, Text};

/// Score display component.
pub struct ScoreDisplay {
    pub position: (f32, f32),
    pub scale: f32,
    score: u32,
    text_buffer: String,
}

impl ScoreDisplay {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            scale: 48.0,
            score: 0,
            text_buffer: String::with_capacity(16),
        }
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = score;
    }

    pub fn render(&mut self, screen_width: f32, screen_height: f32) -> Section<'_> {
        self.text_buffer.clear();
        use std::fmt::Write;
        let _ = write!(self.text_buffer, "{:07}", self.score);

        let scale_ratio = screen_height / 1080.0;
        Section {
            screen_position: self.position,
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.text_buffer)
                    .with_scale(self.scale * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        }
    }
}

/// Combo display component.
pub struct ComboDisplay {
    pub position: (f32, f32),
    pub scale: f32,
    combo: u32,
    text_buffer: String,
}

impl ComboDisplay {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            scale: 64.0,
            combo: 0,
            text_buffer: String::with_capacity(16),
        }
    }

    pub fn set_combo(&mut self, combo: u32) {
        self.combo = combo;
    }

    pub fn render(&mut self, screen_width: f32, screen_height: f32) -> Option<Section<'_>> {
        if self.combo == 0 {
            return None;
        }

        self.text_buffer.clear();
        use std::fmt::Write;
        let _ = write!(self.text_buffer, "{}x", self.combo);

        let scale_ratio = screen_height / 1080.0;
        Some(Section {
            screen_position: self.position,
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.text_buffer)
                    .with_scale(self.scale * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        })
    }
}

/// Accuracy display component.
pub struct AccuracyDisplay {
    pub position: (f32, f32),
    pub scale: f32,
    accuracy: f64,
    text_buffer: String,
}

impl AccuracyDisplay {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            scale: 32.0,
            accuracy: 100.0,
            text_buffer: String::with_capacity(16),
        }
    }

    pub fn set_accuracy(&mut self, accuracy: f64) {
        self.accuracy = accuracy;
    }

    pub fn render(&mut self, screen_width: f32, screen_height: f32) -> Section<'_> {
        self.text_buffer.clear();
        use std::fmt::Write;
        let _ = write!(self.text_buffer, "{:.2}%", self.accuracy);

        let scale_ratio = screen_height / 1080.0;
        Section {
            screen_position: self.position,
            bounds: (screen_width, screen_height),
            text: vec![
                Text::new(&self.text_buffer)
                    .with_scale(self.scale * scale_ratio)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        }
    }
}
